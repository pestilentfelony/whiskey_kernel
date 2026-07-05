#![no_std]
#![no_main]

mod uart;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    uart::init_uart();

    println!("Hello World!");

    if let Some(uart) = uart::get_uart() {
        uart.set_color(uart::COLOR_GREEN);
        print!("Green text");
        uart.reset_color();
        println!("omg im green dadabebadabe");
    }

    loop {
        if let Some(uart) = uart::get_uart() {
            if let Some(byte) = uart.read_byte() {
                if byte == b'\r' {
                    uart.write_byte(b'\n');
                } else {
                    uart.write_byte(byte);
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
