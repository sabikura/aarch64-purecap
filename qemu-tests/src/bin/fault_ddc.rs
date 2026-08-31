//! Negative test. With DDC nulled, a pointer forged from an integer carries no
//! authority: dereferencing it must raise a capability tag data abort.

#![no_std]
#![no_main]

use aarch64_purecap_rt::{entry, exception};
use qemu_tests::{console, println, qemu_exit, UartRegs};

#[entry(grant(
    uart: Mmio<0x0900_0000, UartRegs>,
))]
fn main() -> ! {
    console::init(grant.uart.as_ptr() as *mut u32);
    println!("fault_ddc: start");

    let forged = 0x4100_0000usize as *const u64;
    let value = unsafe { core::ptr::read_volatile(forged) };

    println!("fault_ddc: forged read did not fault, value {:#x}", value);
    qemu_exit(1)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    qemu_tests::panic(info)
}

#[exception(Sync, EL1)]
fn sync_exception() {
    let fault = qemu_tests::read_fault();
    // Data abort without EL change (0x25), capability tag fault (0x28)
    if fault.class == 0x25 && fault.fsc == 0x28 {
        println!("fault_ddc: capability tag fault as expected");
        qemu_exit(0)
    }
    qemu_tests::fault_report_and_exit(&fault, 1)
}
