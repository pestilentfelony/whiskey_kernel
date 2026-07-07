#![no_std]
#![no_main]

mod shell;
mod uart;
mod panic;


#[no_mangle]
pub extern "C" fn rust_main() -> ! {

    uart::init_uart();

    println!("Type 'help' for commands.");

    shell::run_shell();

    loop {}
}
