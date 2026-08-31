//! Positive test. Checks that after entry:
//! - DDC is untagged
//! - PCC is bounded exactly to [__el1_code_start, __el1_code_end)
//! - globals, rodata and function pointers still work through the relocated capabilities
//! - the granted UART capability works
//! Any exception fails the test.

#![no_std]
#![no_main]

use aarch64_purecap_rt::{entry, exception};
use qemu_tests::{cap, code_end, code_start, console, println, qemu_exit, UartRegs};

static GREETING: &str = "hello from rodata";
static FUNC: fn() -> u32 = forty_two;
static mut COUNTER: usize = 0;

fn forty_two() -> u32 {
    42
}

#[entry(grant(
    uart: Mmio<0x0900_0000, UartRegs>,
))]
fn main() -> ! {
    console::init(grant.uart.as_ptr() as *mut u32);
    println!("bounded: start");

    assert!(!cap::tag(cap::ddc()), "ddc still tagged");
    println!("bounded: ddc untagged");

    let pcc = cap::pcc();
    assert_eq!(cap::base(pcc) as usize, code_start(), "pcc base");
    assert_eq!(
        (cap::base(pcc) + cap::length(pcc)) as usize,
        code_end(),
        "pcc limit"
    );
    println!("bounded: pcc bounded to code region");

    unsafe {
        COUNTER += 1;
        assert_eq!(COUNTER, 1, "static mut");
    }
    println!("bounded: {}", GREETING);

    assert_eq!(FUNC(), 42, "fn pointer");
    println!("bounded: fn pointer ok");

    qemu_exit(0)
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    qemu_tests::panic(info)
}

#[exception(Sync, EL1)]
fn sync_exception() {
    qemu_tests::fault_report_and_exit(&qemu_tests::read_fault(), 1)
}
