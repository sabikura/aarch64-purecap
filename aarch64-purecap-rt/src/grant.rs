//! Definition of the types of memory regions/capabilities that are granted
//! to the entry of the runtime.
//!
//! These are used to reduce the need of unsafe Rust to create capabilities out
//! of DDC while keeping track of the exact memory that the program has access
//! to while in safe Rust.

/// Unsafe traits implemented by MMIO register definitions
pub unsafe trait MmioGrant {}

/// Definition of a MMIO region granted to the application
pub struct Mmio<const ADDRESS: usize, T: MmioGrant>(core::marker::PhantomData<T>);

/// Definition of the heap region granted to the application. This specifies the length
/// of the heap arena. The address of where the heap lies should be specified inside the linker
/// script.
pub struct Heap<const SIZE: usize>;

/// Object types range the application has access to through the granted sealing capability.
/// The grant hands over one sealing capability that has the bounds set to the `START` and `END` values.
pub struct SealRange<const START: usize, const END: usize>;
