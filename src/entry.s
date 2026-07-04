.section .text.entry
.global _start

_start:
    # 1. Set up the stack pointer.
    # QEMU's 'virt' machine puts RAM starting at 0x80000000.
    la sp, boot_stack_top

    # 2. Jump to function
    tail rust_main

.section .bss.stack
.global boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 4  # 16KB stack space
.global boot_stack_top
boot_stack_top:
