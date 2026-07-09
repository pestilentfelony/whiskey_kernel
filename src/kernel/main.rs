#![no_std]
#![no_main]

extern crate alloc as _alloc;

use _alloc::string::String;
use crate::alloc::bump_alloc::BumpAllocator;

mod alloc;
mod drivers;
mod panic;
mod shell;
mod trap;


#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Keep main simple..nice and short.

    let mut string_test = String::new();
    string_test.push_str("Hello, World!");



    alloc::bump_alloc::alloc_init();
    drivers::init_drivers();

    println!("Type 'help' for commands.");

    shell::run_shell();
    



    loop {}
}
