use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::println;


pub struct BumpAllocator {
    heap_start: AtomicUsize,
    heap_end: AtomicUsize,
    next: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: AtomicUsize::new(0),
            heap_end: AtomicUsize::new(0),
            next: AtomicUsize::new(0),
        }
    }

    pub unsafe fn init_bump_alloc(&self, heap_start: usize, heap_end: usize) {
        debug_assert!(heap_start <= heap_end);
        self.heap_start.store(heap_start, Ordering::Relaxed);
        self.heap_end.store(heap_end, Ordering::Relaxed);
        self.next.store(heap_start, Ordering::Relaxed);
    }

    pub fn is_initialized(&self) -> bool {
        let start = self.heap_start.load(Ordering::Relaxed);
        let end = self.heap_end.load(Ordering::Relaxed);
        start != 0 && end >= start
    }

    pub fn used_bytes(&self) -> usize {
        let current = self.next.load(Ordering::Relaxed);
        let start = self.heap_start.load(Ordering::Relaxed);
        current.saturating_sub(start)
    }

    pub fn remaining_bytes(&self) -> usize {
        let current = self.next.load(Ordering::Relaxed);
        let end = self.heap_end.load(Ordering::Relaxed);
        end.saturating_sub(current)
    }

    pub fn allocate_raw(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        if size == 0 {
            return ptr::null_mut();
        }

        let heap_start = self.heap_start.load(Ordering::Relaxed);
        if heap_start == 0 {
            return ptr::null_mut();
        }

        let mut current = self.next.load(Ordering::Relaxed);
        let heap_end = self.heap_end.load(Ordering::Relaxed);

        loop {
            let aligned = match current.checked_add(align - 1) {
                Some(value) => value & !(align - 1),
                None => return ptr::null_mut(),
            };

            let alloc_end = match aligned.checked_add(size) {
                Some(end) => end,
                None => return ptr::null_mut(),
            };

            if alloc_end > heap_end {
                return ptr::null_mut();
            }

            match self.next.compare_exchange(
                current,
                alloc_end,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => return aligned as *mut u8,
                Err(next) => current = next,
            }
        }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate_raw(layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Dealloc in a bump allocator...ROFL!!!
    }
}



