use aarch64_purecap_rt_macros::entry;

#[entry]
fn start(grant: Grant) -> ! {
    let _ = grant;
    loop {}
}

fn main() {}
