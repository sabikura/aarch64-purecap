use aarch64_purecap_rt_macros::entry;

#[repr(C)]
struct UartRegisters {
    dr: u32,
}

#[entry(grant(
    heap: Heap<0x10_0000>,
    seal: SealRange<4, 8>,
    uart: Mmio<0x1c09_0000, UartRegisters>,
))]
fn start(grant: Grant) -> ! {
    let _: *mut UartRegisters = grant.uart;
    loop {}
}

fn uart_field(grant: Grant) -> *mut UartRegisters {
    grant.uart
}

fn main() {
    let _ = uart_field;
}
