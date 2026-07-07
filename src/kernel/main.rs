#![no_std]
#![no_main]

mod shell;
mod uart;
mod panic;
mod timer;


#[no_mangle]
pub extern "C" fn rust_main() -> ! {

    uart::init_uart();
    timer::init_timer();

    println!("Type 'help' for commands.");

    shell::run_shell();

    loop {}
}
