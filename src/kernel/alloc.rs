pub mod bump_alloc;
pub mod buddy_alloc;

use println;

extern "C" {
    static _heap_start: u8;
    static _heap_end: u8;
}


#[global_allocator]
//static ALLOCATOR: bump_alloc::BumpAllocator = bump_alloc::BumpAllocator::new();
static ALLOCATOR: buddy_alloc::BuddyAllocator = buddy_alloc::BuddyAllocator::new();


pub fn debug_info() {
    let free_bytes = ALLOCATOR.free_bytes();

    println!("Remaining Bytes:{}",free_bytes);
}

pub fn alloc_init() {
    unsafe {
        let start = &_heap_start as *const u8 as usize;
        let end = &_heap_end as *const u8 as usize;

        ALLOCATOR.init_buddy_alloc(start, end);
    }
}