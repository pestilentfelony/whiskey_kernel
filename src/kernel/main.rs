#![no_std]
#![no_main]

mod shell;
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

    shell::echo_line();

    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut u8, val: i32, count: usize) -> *mut u8 {
    for i in 0..count {
        *dest.add(i) = val as u8;
    }
    dest
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
