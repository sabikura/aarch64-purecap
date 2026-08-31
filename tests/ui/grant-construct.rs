use aarch64_purecap_rt::grant::MmioGrant;
use aarch64_purecap_rt_macros::entry;

#[repr(C)]
struct UartRegisters {
    dr: u32,
}
unsafe impl MmioGrant for UartRegisters {}

#[entry(grant(
    uart: Mmio<0x1c09_0000, UartRegisters>,
))]
fn start(grant: Grant) -> ! {
    let _ = grant;
    loop {}
}

fn main() {
    let _ = Grant {
        uart: core::ptr::null_mut(),
    };
}
