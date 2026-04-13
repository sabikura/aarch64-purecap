use aarch64_purecap_rt::exception;

#[exception(sync, el1)]
fn el1_sync() {}

#[exception(sync, el0)]
fn el0_sync() {}

#[exception(irq, el1)]
fn el1_irq() {}

#[exception(irq, el0)]
fn el0_irq() {}
