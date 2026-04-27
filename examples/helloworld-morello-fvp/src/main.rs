#![no_std]
#![no_main]

use aarch64_purecap_rt::entry;
mod pl011;

#[link_section = ".el2_entry"]
#[used]
#[no_mangle]
pub static EL2_ENTRY_BIN: [u8; include_bytes!(env!("EL2_ENTRY_BIN")).len()] =
    *include_bytes!(env!("EL2_ENTRY_BIN"));

#[entry]
fn main() -> ! {
    pl011::write(b"hello, world!\n\r");
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
