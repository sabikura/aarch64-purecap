//! Rust utilities for CHERI Aarch64 pure-capability targets.
//!
//! # Example
//!
//! ```rust,ignore
//! // TODO: Example
//! ```
#![no_std]

/// Creates a capability derived from the Default Data Capability (DDC) with the DDC's
/// bounds and permissions, and its address set to `address`.
///
/// # Safety
///
/// TODO
pub unsafe fn capability_from_address<T>(address: usize) -> *mut T {
    let ptr: *mut T;
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
