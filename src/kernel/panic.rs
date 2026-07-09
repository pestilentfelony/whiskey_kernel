use core::alloc::Layout;
use core::panic::PanicInfo;
use {print, println};

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}

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
    panic!("Panic induced...for testing purposes.");
}
