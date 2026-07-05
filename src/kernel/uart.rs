const UART_BASE: usize = 0x1000_0000;

const THR: usize = 0; // Transmit Holding Register (write here to send)
const RBR: usize = 0; // Receive Buffer Register (read here to receive)
const LSR: usize = 5; // Line Status Register

const LSR_THRE: u8 = 1 << 5; // "Transmit Holding Register Empty" bit
const LSR_DR: u8   = 1 << 0; // "Data Ready" bit

pub struct Uart {
    base: usize,
}

impl Uart {
    pub const fn new(base: usize) -> Self {
        Uart { base }
    }

    fn reg(&self, offset: usize) -> *mut u8 {
        (self.base + offset) as *mut u8
    }


    pub fn write_byte(&self, byte: u8) {
        unsafe {
            while core::ptr::read_volatile(self.reg(LSR)) & LSR_THRE == 0 {
                // spin, contemplating your life choices
            }
            core::ptr::write_volatile(self.reg(THR), byte);
        }
    }


    pub fn read_byte(&self) -> Option<u8> {
        unsafe {
            if core::ptr::read_volatile(self.reg(LSR)) & LSR_DR != 0 {
                Some(core::ptr::read_volatile(self.reg(RBR)))
            } else {
                None
            }
        }
    }

    pub fn write_str_raw(&self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}


impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str_raw(s);
        Ok(())
    }
}
