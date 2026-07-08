#![no_std]
#![no_main]

mod shell;
mod uart;
mod panic;
mod trap;
mod timer;


#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Keep main simple..nice and short.

    uart::init_uart();
    timer::init_timer();
    trap::enable_interrupts();



    println!("Type 'help' for commands.");

    shell::run_shell();

    loop {}
}
