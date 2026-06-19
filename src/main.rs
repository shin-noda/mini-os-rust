#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Link directly to the symbols defined in our kernel.ld script
extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
}

/// Our true kernel entry point called directly from src/boot.S
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // 1. Initialize the .bss section to 0 (our custom memset equivalent)
    unsafe {
        let bss_start = &raw mut __bss as *mut u8;
        let bss_end = &raw mut __bss_end as *mut u8;
        let mut current = bss_start;
        
        // Loop from __bss to __bss_end and zero out memory using volatile writes
        while current < bss_end {
            current.write_volatile(0);
            current = current.add(1);
        }
    }

    // 2. Fall into our infinite kernel execution loop
    loop {
        // Hint to the RISC-V CPU that we are idling in a spin loop
        core::hint::spin_loop();
    }
}

/// Mandatory panic handler for bare-metal #![no_std]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}