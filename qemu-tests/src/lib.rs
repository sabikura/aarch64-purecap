//! Support code for the QEMU tests: capability inspection, PL011 console,
//! fault decoding and semihosting exit. Builds only for the purecap target.

#![no_std]
#![feature(link_llvm_intrinsics)]

use core::panic::PanicInfo;

/// PL011 base address on the QEMU virt machine
pub const UART0_BASE: usize = 0x0900_0000;

/// PL011 register block. Only the data register is used.
#[repr(C)]
pub struct UartRegs {
    pub dr: u32,
}

// SAFETY: matches the PL011 register layout at the granted address
unsafe impl aarch64_purecap_rt::grant::MmioGrant for UartRegs {}

/// Capability field accessors through the CHERI LLVM intrinsics
pub mod cap {
    mod intrinsics {
        extern "C" {
            #[link_name = "llvm.cheri.ddc.get"]
            pub fn ddc_get() -> *mut u8;
            #[link_name = "llvm.cheri.pcc.get"]
            pub fn pcc_get() -> *mut u8;
            #[link_name = "llvm.cheri.cap.tag.get"]
            pub fn tag_get(cap: *mut u8) -> bool;
            #[link_name = "llvm.cheri.cap.base.get.i64"]
            pub fn base_get(cap: *mut u8) -> u64;
            #[link_name = "llvm.cheri.cap.length.get.i64"]
            pub fn length_get(cap: *mut u8) -> u64;
            #[link_name = "llvm.cheri.cap.address.set.i64"]
            pub fn address_set(cap: *mut u8, address: u64) -> *mut u8;
        }
    }

    pub fn ddc() -> *mut u8 {
        unsafe { intrinsics::ddc_get() }
    }

    pub fn pcc() -> *mut u8 {
        unsafe { intrinsics::pcc_get() }
    }

    pub fn tag(cap: *mut u8) -> bool {
        unsafe { intrinsics::tag_get(cap) }
    }

    pub fn base(cap: *mut u8) -> u64 {
        unsafe { intrinsics::base_get(cap) }
    }

    pub fn length(cap: *mut u8) -> u64 {
        unsafe { intrinsics::length_get(cap) }
    }

    pub fn with_address(cap: *mut u8, address: u64) -> *mut u8 {
        unsafe { intrinsics::address_set(cap, address) }
    }
}

/// Addresses of the PCC region symbols from the runtime linker script
pub fn code_start() -> usize {
    extern "C" {
        static __el1_code_start: u8;
    }
    unsafe { core::ptr::addr_of!(__el1_code_start) as usize }
}

pub fn code_end() -> usize {
    extern "C" {
        static __el1_code_end: u8;
    }
    unsafe { core::ptr::addr_of!(__el1_code_end) as usize }
}

/// Console over the granted PL011 data register
pub mod console {
    static mut UART_DR: *mut u32 = core::ptr::null_mut();

    /// Store the granted capability to the PL011 data register.
    /// Must be called before any print, panic or fault report.
    pub fn init(dr: *mut u32) {
        unsafe { UART_DR = dr };
    }

    pub struct Console;

    impl core::fmt::Write for Console {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let dr = unsafe { UART_DR };
            if !dr.is_null() {
                for byte in s.bytes() {
                    unsafe { core::ptr::write_volatile(dr, byte as u32) };
                }
            }
            Ok(())
        }
    }

    pub fn write_fmt(args: core::fmt::Arguments) {
        use core::fmt::Write;
        let _ = Console.write_fmt(args);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { $crate::console::write_fmt(core::format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {{ $crate::print!($($arg)*); $crate::print!("\n"); }};
}

/// Decoded ESR_EL1 of the current synchronous exception
pub struct Fault {
    /// Exception class, ESR_EL1[31:26]
    pub class: u64,
    /// Fault status code, ESR_EL1[5:0]
    pub fsc: u64,
    pub esr: u64,
}

pub fn read_fault() -> Fault {
    let esr: u64;
    unsafe {
        core::arch::asm!("mrs {esr}, esr_el1", esr = out(reg) esr, options(nomem, nostack));
    }
    Fault {
        class: esr >> 26,
        fsc: esr & 0x3f,
        esr,
    }
}

/// Print the fault and terminate QEMU with the given exit code
pub fn fault_report_and_exit(fault: &Fault, code: u64) -> ! {
    println!(
        "fault: esr {:#018x} class {:#04x} fsc {:#04x}",
        fault.esr, fault.class, fault.fsc
    );
    qemu_exit(code)
}

/// Panic handler body: report and terminate QEMU with exit code 1
pub fn panic(info: &PanicInfo) -> ! {
    println!("panic: {}", info);
    qemu_exit(1)
}

/// Terminate QEMU with the given exit code using semihosting
pub fn qemu_exit(code: u64) -> ! {
    const SYS_EXIT: u64 = 0x18;
    const ADP_STOPPED_APPLICATION_EXIT: u64 = 0x20026;

    let block = [ADP_STOPPED_APPLICATION_EXIT, code];
    loop {
        // SAFETY: semihosting call
        unsafe {
            core::arch::asm!(
                "hlt #0xf000",
                in("x0") SYS_EXIT,
                in("x1") block.as_ptr() as usize,
            );
        }
    }
}
