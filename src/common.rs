use core::fmt;
use crate::sbi;

/// A structural marker representing our system serial console interface
pub struct KernelConsole;

pub type Paddr = u32; // Physical address (32-bit for our RISC-V target)
pub type Vaddr = u32; // Virtual address (32-bit for our RISC-V target)

// Implement the core formatting trait. 
// This acts as the bridge connecting Rust's format strings to our physical UART driver.
impl fmt::Write for KernelConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            sbi::putchar(byte);
        }
        Ok(())
    }
}

/// A hidden helper function that hooks into the macro expansion engine.
/// We mark it public so macros inside other files can see it.
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    // For now, our kernel is single-core and single-threaded, 
    // so we can write directly without any spinlocks or mutexes.
    KernelConsole.write_fmt(args).unwrap();
}

/// Type-safe macro replacing C's variadic printf
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::common::_print(format_args!($($arg)*)));
}

/// Type-safe macro for printing with an automatic trailing newline
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Rounds up `value` to the nearest multiple of `align`. `align` must be a power of 2.
pub const fn align_up(value: usize, align: usize) -> usize {
    // Add (align - 1) to push it to or past the next boundary, 
    // then bitwise-AND with the inverted mask to clear the lower offset bits.
    (value + align - 1) & !(align - 1)
}

/// Checks if `value` is a multiple of `align`. `align` must be a power of 2.
pub const fn is_aligned(value: usize, align: usize) -> bool {
    (value & (align - 1)) == 0
}

/// Copies `n` bytes from `src` to `dst`.
/// 
/// # Safety
/// The caller must ensure that the memory blocks do not overlap and pointers are valid.
#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut d = dst;
    let mut s = src;
    
    for _ in 0..n {
        *d = *s;       // Write the byte from source into destination
        d = d.add(1);  // Explicitly move destination pointer forward by 1 byte
        s = s.add(1);  // Explicitly move source pointer forward by 1 byte
    }
    
    dst
}

/// Fills the first `n` bytes of `buf` with byte `c`.
/// 
/// # Safety
/// The caller must ensure `buf` points to a valid memory region of at least `n` bytes.
#[no_mangle]
pub unsafe extern "C" fn memset(buf: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut p = buf;
    let value = c as u8; // Cast the incoming C integer representation down to a raw byte
    
    for _ in 0..n {
        *p = value;    // Write the value to the current pointer address
        p = p.add(1);  // Explicitly move the pointer forward by 1 byte
    }
    
    buf
}

/// Copies a null-terminated C-string from `src` to `dst`.
/// 
/// # Safety
/// The caller must ensure `dst` has allocated enough memory capacity to prevent
/// a buffer overflow failure.
pub unsafe fn strcpy(dst: *mut u8, src: *const u8) -> *mut u8 {
    let mut d = dst;
    let mut s = src;
    
    // While the byte at the source address is not the null terminator
    while *s != 0 {
        *d = *s;       // Copy the character
        d = d.add(1);  // Shift destination pointer forward
        s = s.add(1);  // Shift source pointer forward
    }
    
    *d = 0; // Explicitly append the trailing null terminator
    dst
}

/// Compares two null-terminated C-strings.
/// Returns 0 if equal, a positive value if s1 > s2, or a negative value if s1 < s2.
/// 
/// # Safety
/// Both pointers must point to valid, null-terminated memory tracks.
pub unsafe fn strcmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut p1 = s1;
    let mut p2 = s2;
    
    // Loop while neither string has hit its null terminator
    while *p1 != 0 && *p2 != 0 {
        if *p1 != *p2 {
            break; // Found a difference, stop scanning
        }
        p1 = p1.add(1);
        p2 = p2.add(1);
    }
    
    // Subtract the unmatched byte values to calculate relative lexicographical order
    (*p1 as i32) - (*p2 as i32)
}

/// Runs a diagnostic suite to verify memory alignment and string manipulation.
pub fn run_tests() {
    crate::println!("\n--- Running Kernel Library Diagnostics ---");
    
    // Test Alignment Bitwise Math
    let unaligned_addr = 0x1234;
    let page_aligned = align_up(unaligned_addr, 0x1000);
    crate::println!("align_up(0x1234, 0x1000) = 0x{:x} (Expected: 0x2000)", page_aligned);
    crate::println!("is_aligned(0x2000, 0x1000) = {}", is_aligned(0x2000, 0x1000));
    crate::println!("is_aligned(0x1234, 0x1000) = {}", is_aligned(0x1234, 0x1000));

    // Test C-String Comparison (\0 terminated)
    let str_a = "shell\0".as_ptr();
    let str_b = "shell\0".as_ptr();
    let str_c = "exit\0".as_ptr();

    unsafe {
        crate::println!("strcmp('shell', 'shell') = {} (Expected: 0)", strcmp(str_a, str_b));
        crate::println!("strcmp('shell', 'exit')  = {} (Expected: positive)", strcmp(str_a, str_c));
    }
    crate::println!("------------------------------------------");
}