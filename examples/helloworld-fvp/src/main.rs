#![no_std]
#![no_main]

mod pl011;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
