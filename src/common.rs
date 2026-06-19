use core::fmt;
use crate::sbi;

/// A structural marker representing our system serial console interface
pub struct KernelConsole;

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