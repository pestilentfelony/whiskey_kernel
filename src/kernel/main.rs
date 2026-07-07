#![no_std]
#![no_main]

mod shell;
mod uart;
mod panic;


#[no_mangle]
pub extern "C" fn rust_main() -> ! {

    uart::init_uart();

    if let Some(uart) = uart::get_uart() {
        uart.set_color(uart::COLOR_GREEN);
        print!("Green text");
        uart.reset_color();
        println!("omg im green dadabebadabe");
    }

    shell::run_shell();

    loop {}
}
