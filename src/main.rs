// src/main.rs

#![no_std]
#![no_main]

pub mod sbi;
pub mod common; 
pub mod panic;

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

    // 2. Fire off our native formatting macros instead of a manual byte loop!
    println!("\n\nHello {}!", "World from Rust Kernel");
    println!("1 + 2 = {}, hex verification: {:x}", 1 + 2, 0x1234abcd);

    // Run our library diagnostics (align_up, strcmp, etc.)
    common::run_tests();

    // 3. Trigger our fresh bare-metal panic test!
    panic!("booted!");

    // Everything below this point is guaranteed unreachable.
    // The compiler knows this because our panic handler returns '!'
}