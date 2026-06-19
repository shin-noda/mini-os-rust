use core::arch::asm;

/// The structural pair mapping the return state from an OpenSBI call.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SbiRet {
    pub error: isize,
    pub value: isize,
}

/// Invokes an OpenSBI environment call via the hardware `ecall` instruction.
#[inline(always)]
pub fn sbi_call(
    arg0: isize, 
    arg1: isize, 
    arg2: isize, 
    arg3: isize, 
    arg4: isize, 
    arg5: isize, 
    fid: isize, 
    eid: isize
) -> SbiRet {
    let mut a0 = arg0;
    let mut a1 = arg1;

    unsafe {
        asm!(
            "ecall",
            inout("a0") a0,
            inout("a1") a1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            options(nostack, nomem)
        );
    }

    SbiRet { error: a0, value: a1 }
}

/// Write a single ASCII byte to the OpenSBI debug console.
pub fn putchar(ch: u8) {
    sbi_call(ch as isize, 0, 0, 0, 0, 0, 0, 1);
}