#![no_std]
#![no_main]

use core::panic::PanicInfo;

// UART0 is mapped exactly at 0x10000000
const UART0: *mut u8 = 0x1000_0000 as *mut u8;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Write "HI" directly to the hardware serial port
    unsafe {
        core::ptr::write_volatile(UART0, b'H');
        core::ptr::write_volatile(UART0, b'I');
        core::ptr::write_volatile(UART0, b'\n');
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
