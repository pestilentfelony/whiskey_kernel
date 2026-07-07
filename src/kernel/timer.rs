use core::ptr::{read_volatile, write_volatile};

const CLINT_BASE: usize = 0x0200_0000;
const MTIMECMP: *mut u64 = (CLINT_BASE + 0x4000) as *mut u64;
const MTIME: *const u64 = (CLINT_BASE + 0x0bff8) as *const u64;
const TIMER_INTERVAL: u64 = 100_000;

static mut TICK_COUNT: u64 = 0;

pub fn init_timer() {
    set_next_timer();
}

fn set_next_timer() {
    unsafe {
        let now = read_volatile(MTIME);
        write_volatile(MTIMECMP, now.wrapping_add(TIMER_INTERVAL));
    }
}

#[no_mangle]
pub extern "C" fn handle_timer_interrupt() {
    unsafe {
        TICK_COUNT = TICK_COUNT.wrapping_add(1);
        set_next_timer();
    }
}

pub fn ticks() -> u64 {
    unsafe { TICK_COUNT }
}
