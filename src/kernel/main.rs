#![no_std]
#![no_main]

mod shell;
mod uart;
mod panic;
mod trap;
mod timer;
mod plic;


#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // Keep main simple..nice and short.

    uart::init_uart();
    
    // Initialize PLIC for hart 0
    plic::init_plic_for_hart(0);
    
    plic::set_priority(10, 1);      // Set UART (IRQ 10) priority to 1
    plic::enable_irq_for_hart(0, 10); // Enable UART interrupt for hart 0
    
    timer::init_timer();
    trap::enable_interrupts();  // Enable all interrupts (timer + external)

    println!("Type 'help' for commands.");

    shell::run_shell();

    loop {}
}
