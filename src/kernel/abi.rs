use crate::{drivers::uart, println};

pub const SYS_WRITE: usize = 1;
pub const SYS_EXIT: usize = 2;
pub const SYS_READ: usize = 3;

#[no_mangle]
pub extern "C" fn rust_syscall_handler(
    syscall: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    mepc: usize,
) -> usize {
    match syscall {
        SYS_WRITE => {
            let fd = arg0;
            let ptr = arg1 as *const u8;
            let len = arg2;

            if fd == 1 && !ptr.is_null() {
                let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };
                if let Some(uart) = uart::get_uart() {
                    uart.write_bytes(bytes);
                }
                len
            } else {
                usize::MAX
            }
        }
        SYS_EXIT => {
            println!("user exit request with status {}", arg0);
            loop {
                unsafe {
                    core::arch::asm!("wfi");
                }
            }
        }
        SYS_READ => {
            let fd = arg0;
            let ptr = arg1 as *mut u8;
            let len = arg2;

            if fd == 0 && !ptr.is_null() && len > 0 {
                if let Some(uart) = uart::get_uart() {
                    if let Some(byte) = uart.read_byte() {
                        unsafe {
                            core::ptr::write(ptr, byte);
                        }
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            } else {
                0
            }
        }
        _ => {
            println!("unknown syscall {} at {:#x}", syscall, mepc);
            usize::MAX
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syscall_numbers_are_defined() {
        assert_eq!(SYS_WRITE, 1);
        assert_eq!(SYS_EXIT, 2);
        assert_eq!(SYS_READ, 3);
    }
}
