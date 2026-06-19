// src/main.rs

#![no_std]
#![no_main]

pub mod sbi;
pub mod common; 
pub mod panic;
pub mod trap;

// Link directly to the symbols defined in our linker script and assembly files
extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
    
    // Bring in our exception entry point symbol from src/boot.S
    fn kernel_entry();
}

/// Our true kernel entry point called directly from src/boot.S
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // 1. Initialize the .bss section to 0 using volatile writes
    unsafe {
        let bss_start = &raw mut __bss as *mut u8;
        let bss_end = &raw mut __bss_end as *mut u8;
        let mut current = bss_start;
        
        while current < bss_end {
            current.write_volatile(0);
            current = current.add(1);
        }
    }

    // 2. Fire off our native formatting macros
    println!("\n\nHello {}!", "World from Rust Kernel");
    println!("1 + 2 = {}, hex verification: {:x}", 1 + 2, 0x1234abcd);

    // Run our library diagnostics (align_up, strcmp, etc.)
    common::run_tests();

    // 3. Register our exception handler and trigger the trap smoke test!
    println!("Registering exception handler via stvec...");
    unsafe {
        // Point stvec directly to the starting memory address of our assembly loop
        trap::write_stvec(kernel_entry as *const () as u32);
        
        println!("Triggering illegal instruction exception via 'unimp'...");
        core::arch::asm!("unimp");
    }

    // This point is now mathematically unreachable because the CPU will trap and panic!
    loop {}
}