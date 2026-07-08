#![no_std]
#![no_main]


mod shell;
mod panic;
mod trap;
mod alloc;
mod drivers;


#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Keep main simple..nice and short.

    drivers::init_drivers();


    println!("Type 'help' for commands.");

    shell::run_shell();

    loop {}
}
