//! Bump allocator implementation. Should be used in pair with the heap grant received through the
//! `entry` macro.
//!
//! ```ignore
//! #[global_allocator]
//! static ALLOCATOR: BumpAllocator = BumpAllocator::new();
//! const HEAP_SIZE: usize = 0x10_0000;
//!
//! #[entry(grant(heap: Heap<HEAP_SIZE>))]
//! fn main(grant: Grant) -> ! {
//!     unsafe { ALLOCATOR.init(grant.heap, HEAP_SIZE) };
//!     let boxed = alloc::boxed::Box::new(42);
//!     ...
//! }
//! ```

use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Simple bump allocator
pub struct BumpAllocator {
    base: UnsafeCell<*mut u8>,
    size: AtomicUsize,
    next: AtomicUsize,
}

// SAFETY: `base` is written once by `init` before the first allocation, only read
// afterwards
unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    /// Create a new [`BumpAllocator`] instance
    pub const fn new() -> Self {
        Self {
            base: UnsafeCell::new(ptr::null_mut()),
            size: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
        }
    }

    /// Initialize the allocator with the heap pointer
    ///
    /// # Safety
    ///
    /// Must be called at most once, before the first allocation. `heap` must be the
    /// heap grant pointer and `size` the length of the granted arena.
    pub unsafe fn init(&self, heap: *mut u8, size: usize) {
        *self.base.get() = heap;
        self.size.store(size, Ordering::Release);
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = self.size.load(Ordering::Acquire);
        let base = *self.base.get();
        if base.is_null() {
            return ptr::null_mut();
        }

        let mut offset = 0;
        let claimed = self
            .next
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
                // Layout guarantees the alignment is a power of two
                let aligned = next.checked_add(layout.align() - 1)? & !(layout.align() - 1);
                let end = aligned.checked_add(layout.size())?;
                if end > size {
                    return None;
                }
                offset = aligned;
                Some(end)
            });

        match claimed {
            Ok(_) => base.add(offset),
            Err(_) => ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Does nothing
    }
}
