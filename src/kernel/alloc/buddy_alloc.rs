use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::sync::atomic::{AtomicBool, Ordering};

/* TODO:
disable interrupts while lock is held
assert heap start > 0 
switch to a double linked intrusive list 
 */

/// Number of order-buckets we keep free lists for
const NUM_ORDERS: usize = 32;

/// Smallest block the allocator will ever hand out
const MIN_BLOCK_SIZE: usize = 64;

// Spinlock implemented for buddy_alloc

pub struct Spinlock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Spinlock<T> {}
unsafe impl<T: Send> Send for Spinlock<T> {}

pub struct SpinlockGuard<'a, T> {
    lock: &'a Spinlock<T>,
}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Spinlock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinlockGuard<'_, T> {
        // Try to grab it
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) {
                core::hint::spin_loop();
            }
        }
        SpinlockGuard { lock: self }
    }
}

impl<'a, T> Deref for SpinlockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for SpinlockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for SpinlockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}


struct BuddyState {
    heap_start: usize,
    heap_end: usize,
    max_order: usize,
    free_lists: [usize; NUM_ORDERS],
}

impl BuddyState {
    const fn new() -> Self {
        BuddyState {
            heap_start: 0,
            heap_end: 0,
            max_order: 0,
            free_lists: [0; NUM_ORDERS],
        }
    }

    #[inline]
    fn read_next(addr: usize) -> usize {
        unsafe { ptr::read(addr as *const usize) }
    }

    #[inline]
    fn write_next(addr: usize, next: usize) {
        unsafe { ptr::write(addr as *mut usize, next) };
    }

    fn push_free(&mut self, order: usize, addr: usize) {
        Self::write_next(addr, self.free_lists[order]);
        self.free_lists[order] = addr;
    }

    fn pop_free(&mut self, order: usize) -> Option<usize> {
        let head = self.free_lists[order];
        if head == 0 {
            return None;
        }
        self.free_lists[order] = Self::read_next(head);
        Some(head)
    }

    /// remove a specific address from an orders free list
    fn remove_free(&mut self, order: usize, target: usize) -> bool {
        let mut cur = self.free_lists[order];
        if cur == target {
            self.free_lists[order] = Self::read_next(cur);
            return true;
        }
        while cur != 0 {
            let next = Self::read_next(cur);
            if next == target {
                let after_target = Self::read_next(target);
                Self::write_next(cur, after_target);
                return true;
            }
            cur = next;
        }
        false
    }

    #[inline]
    fn block_size(order: usize) -> usize {
        MIN_BLOCK_SIZE << order
    }
    fn buddy_of(&self, addr: usize, order: usize) -> usize {
        let offset = addr - self.heap_start;
        let size = Self::block_size(order);
        self.heap_start + (offset ^ size)
    }

    fn order_for(&self, layout: Layout) -> Option<usize> {
        let needed = layout.size().max(layout.align()).max(MIN_BLOCK_SIZE);
        let rounded = needed.next_power_of_two();
        let order = (rounded.trailing_zeros() - MIN_BLOCK_SIZE.trailing_zeros()) as usize;
        if order <= self.max_order {
            Some(order)
        } else {
            None
        }
    }

    fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let order = match self.order_for(layout) {
            Some(o) => o,
            None => return ptr::null_mut(),
        };

        // Smallest non-empty free list at or above `order`.
        let mut found_order = None;
        for o in order..=self.max_order {
            if self.free_lists[o] != 0 {
                found_order = Some(o);
                break;
            }
        }

        let mut current_order = match found_order {
            Some(o) => o,
            None => return ptr::null_mut(), // genuinely out of memory
        };

        let mut block = self.pop_free(current_order).unwrap();

        while current_order > order {
            current_order -= 1;
            let buddy = block + Self::block_size(current_order);
            self.push_free(current_order, buddy);
        }

        block as *mut u8
    }

    fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let mut addr = ptr as usize;
        let mut order = match self.order_for(layout) {
            Some(o) => o,
            None => return, 
        };

        // Try to merge upward as far as possible
        while order < self.max_order {
            let buddy = self.buddy_of(addr, order);
            if self.remove_free(order, buddy) {
                addr = addr.min(buddy);
                order += 1;
            } else {
                break;
            }
        }

        self.push_free(order, addr);
    }
}


pub struct BuddyAllocator {
    inner: Spinlock<BuddyState>,
}

impl BuddyAllocator {
    pub const fn new() -> Self {
        BuddyAllocator {
            inner: Spinlock::new(BuddyState::new()),
        }
    }

    pub unsafe fn init_buddy_alloc(&self, heap_start: usize, heap_end: usize) {
        debug_assert!(heap_start <= heap_end);

        let aligned_start = (heap_start + MIN_BLOCK_SIZE - 1) & !(MIN_BLOCK_SIZE - 1);
        let aligned_end = heap_end & !(MIN_BLOCK_SIZE - 1);

        let mut state = self.inner.lock();
        state.heap_start = aligned_start;
        state.heap_end = aligned_end;

        if aligned_end <= aligned_start {
            state.max_order = 0;
            return;
        }

        let total = aligned_end - aligned_start;

        let mut max_order = 0usize;
        while max_order + 1 < NUM_ORDERS && BuddyState::block_size(max_order + 1) <= total {
            max_order += 1;
        }
        state.max_order = max_order;
        let mut offset = 0usize;
        while offset < total {
            let remaining = total - offset;
            let mut order = max_order;
            loop {
                let size = BuddyState::block_size(order);
                if size <= remaining && offset % size == 0 {
                    break;
                }
                if order == 0 {
                    break;
                }
                order -= 1;
            }
            let size = BuddyState::block_size(order);
            state.push_free(order, aligned_start + offset);
            offset += size;
        }
    }

    pub fn is_initialized(&self) -> bool {
        let state = self.inner.lock();
        state.heap_start != 0 && state.heap_end >= state.heap_start
    }

    pub fn free_bytes(&self) -> usize {
        let state = self.inner.lock();
        let mut total = 0usize;
        for order in 0..=state.max_order {
            let mut cur = state.free_lists[order];
            let size = BuddyState::block_size(order);
            while cur != 0 {
                total += size;
                cur = BuddyState::read_next(cur);
            }
        }
        total
    }
}

unsafe impl GlobalAlloc for BuddyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.inner.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() {
            return;
        }
        self.inner.lock().dealloc(ptr, layout);
    }
}