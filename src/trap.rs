// src/trap.rs

/// The program execution state saved on the stack during a hardware exception.
/// Forced to standard C-representation to ensure predictable field sequencing.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TrapFrame {
    pub ra: u32,  pub gp: u32,  pub tp: u32,  pub t0: u32,
    pub t1: u32,  pub t2: u32,  pub t3: u32,  pub t4: u32,
    pub t5: u32,  pub t6: u32,  pub a0: u32,  pub a1: u32,
    pub a2: u32,  pub a3: u32,  pub a4: u32,  pub a5: u32,
    pub a6: u32,  pub a7: u32,  pub s0: u32,  pub s1: u32,
    pub s2: u32,  pub s3: u32,  pub s4: u32,  pub s5: u32,
    pub s6: u32,  pub s7: u32,  pub s8: u32,  pub s9: u32,
    pub s10: u32, pub s11: u32, pub sp: u32,
}

/// Helper function to read the Supervisor Cause CSR
#[inline(always)]
pub fn read_scause() -> u32 {
    let value: u32;
    unsafe {
        core::arch::asm!("csrr {}, scause", out(reg) value);
    }
    value
}

/// Helper function to read the Supervisor Trap Value CSR
#[inline(always)]
pub fn read_stval() -> u32 {
    let value: u32;
    unsafe {
        core::arch::asm!("csrr {}, stval", out(reg) value);
    }
    value
}

/// Helper function to read the Supervisor Exception Program Counter CSR
#[inline(always)]
pub fn read_sepc() -> u32 {
    let value: u32;
    unsafe {
        core::arch::asm!("csrr {}, sepc", out(reg) value);
    }
    value
}

/// Writes a target address directly to the Supervisor Trap Vector CSR
#[inline(always)]
pub unsafe fn write_stvec(address: u32) {
    core::arch::asm!("csrw stvec, {}", in(reg) address);
}

/// The high-level Rust exception entry point called directly from assembly.
/// Receives a raw pointer to the TrapFrame allocated on the stack.
#[no_mangle]
pub extern "C" fn handle_trap(_f: *mut TrapFrame) {
    let scause = read_scause();
    let stval = read_stval();
    let sepc = read_sepc();

    // Fire off our custom panic handler to freeze the CPU and print the crash dump
    panic!(
        "unexpected trap scause={:#010x}, stval={:#010x}, sepc={:#010x}",
        scause, stval, sepc
    );
}