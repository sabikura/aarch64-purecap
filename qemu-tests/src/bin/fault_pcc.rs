//! Negative test. Branching to an address outside [__el1_code_start,
//! __el1_code_end) must raise a capability instruction abort, because every
//! executable capability in the system is bounded to the code region.

#![no_std]
#![no_main]

use aarch64_purecap_rt::{entry, exception};
use qemu_tests::{cap, code_end, console, println, qemu_exit, UartRegs};

#[entry(grant(
    uart: Mmio<0x0900_0000, UartRegs>,
))]
fn main() -> ! {
    console::init(grant.uart.as_ptr() as *mut u32);
    println!("fault_pcc: start");

    let target = cap::with_address(cap::pcc(), (code_end() + 0x100) as u64);
    let f: extern "C" fn() = unsafe { core::mem::transmute(target) };
    f();

    println!("fault_pcc: out of bounds branch did not fault");
    qemu_exit(1)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    qemu_tests::panic(info)
}

#[exception(Sync, EL1)]
fn sync_exception() {
    let fault = qemu_tests::read_fault();
    // Instruction abort without EL change (0x21), capability tag/sealed/bounds/
    // permission fault (0x28..0x2b)
    if fault.class == 0x21 && (0x28..=0x2b).contains(&fault.fsc) {
        println!("fault_pcc: capability instruction abort as expected");
        qemu_exit(0)
    }
    qemu_tests::fault_report_and_exit(&fault, 1)
}
