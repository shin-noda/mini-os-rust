use core::panic::PanicInfo;
use crate::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n!!! KERNEL PANIC !!!");
    
    if let Some(location) = info.location() {
        println!("Location: {}:{}:{}", location.file(), location.line(), location.column());
    }
    
    // PanicMessage implements Display directly, no Option unwrap needed!
    println!("Message: {}", info.message());

    loop {
        core::hint::spin_loop();
    }
}