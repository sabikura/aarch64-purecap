//! Definition of the types of memory regions/capabilities that are granted
//! to the entry of the runtime.
//!
//! These are used to reduce the need of unsafe Rust to create capabilities out
//! of DDC while keeping track of the exact memory that the program has access
//! to while in safe Rust.
//!
//! Each grant type derives its capability from DDC in `from_ddc`. The `entry`
//! macro calls `from_ddc` for every granted item and then nulls DDC, so the
//! grants are the only remaining authority over their regions.

/// Permission bits of a Morello capability, used as "keep" masks for
/// `llvm.cheri.cap.perms.and`.
mod perms {
    pub const GLOBAL: u64 = 1 << 0;
    pub const UNSEAL: u64 = 1 << 10;
    pub const SEAL: u64 = 1 << 11;
    pub const STORE_LOCAL_CAP: u64 = 1 << 12;
    pub const STORE_CAP: u64 = 1 << 13;
    pub const LOAD_CAP: u64 = 1 << 14;
    pub const STORE: u64 = 1 << 16;
    pub const LOAD: u64 = 1 << 17;
}

#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
mod intrinsics {
    extern "C" {
        #[link_name = "llvm.cheri.ddc.get"]
        pub fn ddc_get() -> *mut u8;

        #[link_name = "llvm.cheri.cap.address.set.i64"]
        pub fn address_set(cap: *mut u8, address: u64) -> *mut u8;

        #[link_name = "llvm.cheri.cap.bounds.set.i64"]
        pub fn bounds_set(cap: *mut u8, len: u64) -> *mut u8;

        #[link_name = "llvm.cheri.cap.perms.and.i64"]
        pub fn perms_and(cap: *mut u8, mask: u64) -> *mut u8;
    }
}

/// Derive a capability from DDC with the given address, length and kept permissions.
///
/// # Safety
/// DDC must still be valid and cover `[address, address + len)`.
#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
unsafe fn derive_from_ddc(address: usize, len: usize, keep: u64) -> *mut u8 {
    let cap = intrinsics::ddc_get();
    let cap = intrinsics::address_set(cap, address as u64);
    let cap = intrinsics::bounds_set(cap, len as u64);
    intrinsics::perms_and(cap, keep)
}

#[cfg(not(all(target_arch = "aarch64", target_abi = "purecap")))]
unsafe fn derive_from_ddc(_address: usize, _len: usize, _keep: u64) -> *mut u8 {
    unimplemented!("grants can only be derived on the purecap target")
}

/// Unsafe traits implemented by MMIO register definitions
pub unsafe trait MmioGrant {}

/// Definition of a MMIO region granted to the application.
/// Holds a RW capability without capability permissions, bounded to `size_of::<T>()`.
pub struct Mmio<const ADDRESS: usize, T: MmioGrant> {
    ptr: *mut T,
}

impl<const ADDRESS: usize, T: MmioGrant> Mmio<ADDRESS, T> {
    /// Derive the MMIO capability from DDC.
    ///
    /// # Safety
    /// DDC must still be valid. `ADDRESS` must be the base of a MMIO region
    /// matching the layout of `T`.
    pub unsafe fn from_ddc() -> Self {
        let keep = perms::GLOBAL | perms::LOAD | perms::STORE;
        Self {
            ptr: derive_from_ddc(ADDRESS, core::mem::size_of::<T>(), keep) as *mut T,
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }
}

/// Definition of the heap region granted to the application. This specifies the length
/// of the heap arena. The start of the heap is the `__el1_heap_start` linker symbol.
/// Holds a RW capability with capability permissions, bounded to `SIZE`.
pub struct Heap<const SIZE: usize> {
    ptr: *mut u8,
}

impl<const SIZE: usize> Heap<SIZE> {
    /// Derive the heap capability from DDC.
    ///
    /// # Safety
    /// DDC must still be valid. `[__el1_heap_start, __el1_heap_start + SIZE)` must be
    /// backed by RAM and not overlap anything else.
    pub unsafe fn from_ddc() -> Self {
        extern "C" {
            static __el1_heap_start: u8;
        }
        let start = core::ptr::addr_of!(__el1_heap_start) as usize;
        let keep = perms::GLOBAL
            | perms::LOAD
            | perms::STORE
            | perms::LOAD_CAP
            | perms::STORE_CAP
            | perms::STORE_LOCAL_CAP;
        Self {
            ptr: derive_from_ddc(start, SIZE, keep),
        }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    pub const fn size(&self) -> usize {
        SIZE
    }
}

/// Object types range the application has access to through the granted sealing capability.
/// The grant hands over one sealing capability that has the bounds set to the `START` and `END` values.
pub struct SealRange<const START: usize, const END: usize> {
    cap: *mut u8,
}

impl<const START: usize, const END: usize> SealRange<START, END> {
    /// Derive the sealing capability from DDC.
    ///
    /// # Safety
    /// DDC must still be valid and hold the Seal and Unseal permissions.
    pub unsafe fn from_ddc() -> Self {
        let keep = perms::GLOBAL | perms::SEAL | perms::UNSEAL;
        Self {
            cap: derive_from_ddc(START, END - START, keep),
        }
    }

    pub fn as_ptr(&self) -> *mut u8 {
        self.cap
    }
}
