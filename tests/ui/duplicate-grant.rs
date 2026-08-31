use aarch64_purecap_rt_macros::entry;

#[entry(grant(
    uart: Mmio<0x1c09_0000, UartRegisters>,
    uart: Mmio<0x1f09_0000, UartRegisters>,
))]
fn start(grant: Grant) -> ! {
    let _ = grant;
    loop {}
}

fn main() {}
