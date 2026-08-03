//! Rust utilities for CHERI Aarch64 pure-capability targets.
#![no_std]

/// Creates a capability derived from the Default Data Capability (DDC) with the DDC's
/// bounds and permissions, and its address set to `address`.
pub fn capability_from_address<T>(address: usize) -> *mut T {
    let ptr: *mut T;
    // SAFETY: This is a wrapper over the `cvtd` instruction. This will clear the tag if
    // the data capability is sealed and will result in a fault if dereferenced.
    unsafe {
        core::arch::asm!(
            "cvtd {cap:x}, {addr}",
            cap = out(reg) ptr,
            addr = in(reg) address,
            options(nomem, nostack, preserves_flags),
        );
    }

    ptr
}
