
use core::sync::atomic::{AtomicUsize, Ordering};

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
        self.heap_start.store(heap_start, Ordering::Relaxed);
        self.heap_end.store(heap_end, Ordering::Relaxed);
        self.next.store(heap_start, Ordering::Relaxed);
    }

    pub fn used_bytes(&self) -> usize {
        let current = self.next.load(Ordering::Relaxed);
        let start = self.heap_start.load(Ordering::Relaxed);
        current - start
    }
}