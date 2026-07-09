#![no_std]
#![no_main]

use crate::alloc::bump_alloc::BumpAllocator;

mod alloc;
mod drivers;
mod panic;
mod shell;
mod trap;

extern "C" {
    static _heap_start: u8;
    static _heap_end: u8;
}

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Keep main simple..nice and short.

    alloc::bump_alloc::alloc_init();
    drivers::init_drivers();

    println!("Type 'help' for commands.");

    shell::run_shell();

    loop {}
}
