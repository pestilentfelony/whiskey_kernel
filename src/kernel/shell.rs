/*The Serial Shell
"What the fuck"
*/

use crate::uart;
use {print, println};

pub fn echo_line() {
    let mut buffer = [0u8; 128];
    let mut len = 0usize;

    print_prompt();

    loop {
        if let Some(uart) = uart::get_uart() {
            if let Some(byte) = uart.read_byte() {
                match byte {
                    b'\r' => {
                        uart.write_byte(b'\n');
                        if let Some(cmd) = buffer.get(..len) {
                            handle_command(cmd);
                        }
                        len = 0;
                        print_prompt();
                    }
                    0x08 | 0x7f => {
                        if len > 0 {
                            len -= 1;
                            uart.write_byte(0x08);
                            uart.write_byte(b' ');
                            uart.write_byte(0x08);
                        }
                    }
                    _ => {
                        if len < buffer.len() {
                            buffer[len] = byte;
                            len += 1;
                            uart.write_byte(byte);
                        }
                    }
                }
            }
        }
    }
}

fn handle_command(cmd: &[u8]) {
    match cmd {
        b"help" => {
            println!("Available commands:");
            println!("help -> show this message");
            println!("version -> show the version information");

        }
        b"version" => {
            println!("whiskey_os v{}", option_env!("VERSION").unwrap_or("unknown"));
        }
        b"" => {}
        _ => {
            println!("Unknown command.");
        }
    }
}

fn print_prompt() {
    print!("# ");
}
