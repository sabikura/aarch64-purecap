pub fn write(text: &[u8]) {
    let uart_dr: *mut u32;
    unsafe {
        core::arch::asm!(
            "cvtd {cap:x}, {addr}",
            cap = out(reg) uart_dr,
            addr = in(reg) 0x2A40_0000_usize,
            options(nomem, nostack, preserves_flags),
        );
        for b in text {
            core::ptr::write_volatile(uart_dr, *b as u32);
        }
    }
}
