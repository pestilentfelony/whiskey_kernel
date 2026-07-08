pub mod plic;
pub mod timer;
pub mod uart;

use crate::trap;

pub fn init_drivers() {
    uart::init_uart();

    // Initialize PLIC for hart 0
    plic::init_plic_for_hart(0);

    plic::set_priority(10, 1); // Set UART (IRQ 10) priority to 1
    plic::enable_irq_for_hart(0, 10); // Enable UART interrupt for hart 0

    timer::init_timer();
    trap::enable_interrupts();
}
