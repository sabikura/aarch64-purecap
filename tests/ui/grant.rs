use aarch64_purecap_rt_macros::entry;

struct UartRegisters;

#[entry(grant(
    heap: Heap<0x10_0000>,
    seal: SealRange<4, 8>,
    uart: Mmio<0x1c09_0000, UartRegisters>,
))]
fn start(grant: Grant) -> ! {
    let _ = grant;
    loop {}
}

fn main() {
    let _: Option<Grant> = None;
}
