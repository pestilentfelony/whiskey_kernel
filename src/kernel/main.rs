#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc as _alloc;
mod alloc;
mod drivers;
mod panic;
mod shell;
mod trap;
mod tests;



#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Keep main simple..nice and short.

    alloc::bump_alloc::alloc_init();
    drivers::init_drivers();

    println!("Type 'help' for commands.");

    shell::run_shell();
    
    loop {}
}
