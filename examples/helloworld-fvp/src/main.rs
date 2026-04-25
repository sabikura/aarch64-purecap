#![no_std]
#![no_main]

use aarch64_purecap_rt::entry;
mod exceptions;
mod pl011;

#[entry]
fn main() -> ! {
    pl011::write(b"hello, world!\n\r");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
