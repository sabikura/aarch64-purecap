#![no_std]
#![no_main]

use aarch64_purecap_rt::entry;

#[link_section = ".el2_entry"]
#[used]
#[no_mangle]
pub static EL2_ENTRY_BIN: [u8; include_bytes!(env!("EL2_ENTRY_BIN")).len()] =
    *include_bytes!(env!("EL2_ENTRY_BIN"));

#[entry]
fn main() -> ! {
    write_str(b"hello, world!\n\r");
    loop {}
}

/// Write a slice of bytes to the PL011 UART
fn write_str(data: &[u8]) {
    const UART_PL011_DATA_REGISTER: usize = 0x2A40_0000;

    unsafe {
        let uart_dr = aarch64_purecap_cpu::capability_from_address(UART_PL011_DATA_REGISTER);
        for byte in data {
            core::ptr::write_volatile(uart_dr, *byte as u32);
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
