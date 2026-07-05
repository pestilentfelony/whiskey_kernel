#![no_std]
#![no_main]

mod uart;

use core::panic::PanicInfo;

use crate::uart::Uart;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    Uart::new(0x1000_0000).write_str_raw("Hello, world!\n");

    loop {}
}




#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
