.attribute arch, "rv32imac"
.section .text.boot
.global _start

_start:
    # 1. Load the address of __stack_top (from kernel.ld) into the stack pointer (sp)
    la sp, __stack_top

    # 2. Jump directly into our Rust kernel main function
    j kernel_main

.global kernel_entry
.align 4
kernel_entry:
    # 1. Store the original stack pointer temporarily into sscratch
    csrw sscratch, sp
    
    # 2. Allocate 124 bytes (31 registers * 4 bytes) on the stack for TrapFrame
    addi sp, sp, -124
    
    # 3. Save all general-purpose registers into their relative struct offsets
    sw ra,  4 * 0(sp)
    sw gp,  4 * 1(sp)
    sw tp,  4 * 2(sp)
    sw t0,  4 * 3(sp)
    sw t1,  4 * 4(sp)
    sw t2,  4 * 5(sp)
    sw t3,  4 * 6(sp)
    sw t4,  4 * 7(sp)
    sw t5,  4 * 8(sp)
    sw t6,  4 * 9(sp)
    sw a0,  4 * 10(sp)
    sw a1,  4 * 11(sp)
    sw a2,  4 * 12(sp)
    sw a3,  4 * 13(sp)
    sw a4,  4 * 14(sp)
    sw a5,  4 * 15(sp)
    sw a6,  4 * 16(sp)
    sw a7,  4 * 17(sp)
    sw s0,  4 * 18(sp)
    sw s1,  4 * 19(sp)
    sw s2,  4 * 20(sp)
    sw s3,  4 * 21(sp)
    sw s4,  4 * 22(sp)
    sw s5,  4 * 23(sp)
    sw s6,  4 * 24(sp)
    sw s7,  4 * 25(sp)
    sw s8,  4 * 26(sp)
    sw s9,  4 * 27(sp)
    sw s10, 4 * 28(sp)
    sw s11, 4 * 29(sp)
    
    # 4. Retrieve the original stack pointer from sscratch and save it as the final field
    csrr a0, sscratch
    sw a0,  4 * 30(sp)
    
    # 5. Set up the first argument (a0) to point to our stack frame, then cross over to Rust
    mv a0, sp
    call handle_trap
    
    # 6. (Unreachable if handle_trap panics, but required for structural balance)
    # Restore registers
    lw ra,  4 * 0(sp)
    lw gp,  4 * 1(sp)
    lw tp,  4 * 2(sp)
    lw t0,  4 * 3(sp)
    lw t1,  4 * 4(sp)
    lw t2,  4 * 5(sp)
    lw t3,  4 * 6(sp)
    lw t4,  4 * 7(sp)
    lw t5,  4 * 8(sp)
    lw t6,  4 * 9(sp)
    lw a0,  4 * 10(sp)
    lw a1,  4 * 11(sp)
    lw a2,  4 * 12(sp)
    lw a3,  4 * 13(sp)
    lw a4,  4 * 14(sp)
    lw a5,  4 * 15(sp)
    lw a6,  4 * 16(sp)
    lw a7,  4 * 17(sp)
    lw s0,  4 * 18(sp)
    lw s1,  4 * 19(sp)
    lw s2,  4 * 20(sp)
    lw s3,  4 * 21(sp)
    lw s4,  4 * 22(sp)
    lw s5,  4 * 23(sp)
    lw s6,  4 * 24(sp)
    lw s7,  4 * 25(sp)
    lw s8,  4 * 26(sp)
    lw s9,  4 * 27(sp)
    lw s10, 4 * 28(sp)
    lw s11, 4 * 29(sp)
    
    # Restore the stack pointer itself and jump back
    lw sp,  4 * 30(sp)
    sret