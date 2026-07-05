#![no_std]
#![no_main]

use core::panic::PanicInfo;
const UART: *mut u8 = 0x1000_0000 as *mut u8;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    for &b in b"HI\n" {
        unsafe {
            core::ptr::write_volatile(UART, b);
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}