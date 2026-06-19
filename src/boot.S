.attribute arch, "rv32imac"
.section .text.boot
.global _start

_start:
    # 1. Load the address of __stack_top (from kernel.ld) into the stack pointer (sp)
    la sp, __stack_top

    # 2. Jump directly into our Rust kernel main function
    j kernel_main