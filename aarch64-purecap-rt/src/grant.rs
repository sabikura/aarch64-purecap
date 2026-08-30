//! Definition of the types of memory regions/capabilities that are granted
//! to the entry of the runtime.
//!
//! These are used to reduce the need of unsafe Rust to create capabilities out
//! of DDC while keeping track of the exact memory that the program has access
//! to while in safe Rust.

/// Definition of a MMIO region granted to the application
pub struct Mmio<const ADDRESS: usize, const LEN: usize>;

// pub struct
