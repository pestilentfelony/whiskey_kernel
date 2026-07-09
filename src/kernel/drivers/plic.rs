/* PLIC: Platform level Interrupt Controller
The PLIC lets the kernel assign priority levels depending on urgency. */


use core::ptr::{read_volatile, write_volatile};

const PLIC_BASE: usize = 0x0C00_0000;
const PLIC_PRIORITY: usize = PLIC_BASE + 0x0;
const PLIC_PENDING: usize = PLIC_BASE + 0x1000;
const PLIC_ENABLE: usize = PLIC_BASE + 0x2000;
const PLIC_CONTEXT_BASE: usize = PLIC_BASE + 0x200000;


// Harts: Hardware threads. One hart = one logical CPU.
pub fn init_plic_for_hart(hartid: usize) {
    unsafe {
        let threshold = (PLIC_CONTEXT_BASE + hartid * 0x1000) as *mut u32;
        write_volatile(threshold, 0);
    }
}

pub fn set_priority(irq: usize, prio: u32) {
    unsafe {
        let p = (PLIC_PRIORITY + irq * 4) as *mut u32;
        write_volatile(p, prio);
    }
}


/*
irq: interrupt request.
it is the signal sent to actually perform the interrupt. this function enables this. */
pub fn enable_irq_for_hart(hartid: usize, irq: usize) {
    unsafe {
        let byte_off = (irq / 32) * 4;
        let bit = 1u32 << (irq % 32);
        let en = (PLIC_ENABLE + hartid * 0x100 + byte_off) as *mut u32;
        let v = read_volatile(en);
        write_volatile(en, v | bit);
    }
}

pub fn claim(hartid: usize) -> u32 {
    unsafe {
        let claim = (PLIC_CONTEXT_BASE + hartid * 0x1000 + 0x4) as *mut u32;
        read_volatile(claim)
    }
}

pub fn complete(hartid: usize, irq: u32) {
    unsafe {
        let claim = (PLIC_CONTEXT_BASE + hartid * 0x1000 + 0x4) as *mut u32;
        write_volatile(claim, irq);
    }
}
