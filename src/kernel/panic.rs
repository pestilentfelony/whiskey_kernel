use core::panic::PanicInfo;
use {print, println};

// Don't panic, I got it under control..not.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "PANIC at {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        println!("PANIC: no location available");
    }

    println!("{}", info.message());

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

pub fn induce_panic() {
    panic!("SCRAM!!!");
}