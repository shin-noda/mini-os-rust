// src/panic.rs
use core::panic::PanicInfo;
use crate::println; 

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 1. Match the "PANIC: " prefix and location from the tutorial
    if let Some(location) = info.location() {
        println!("PANIC: {}:{}: ", location.file(), location.line());
    } else {
        println!("PANIC: unknown_file:0: ");
    }

    // 2. Print the message directly (No 'if let Some' or unwrapping needed!)
    println!("{}", info.message());

    // 3. The infinite halt loop
    loop {
        core::hint::spin_loop();
    }
}