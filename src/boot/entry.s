.section .text.entry
.global _start

_start:
    # 1. Set up the stack pointer.
    # QEMU's 'virt' machine puts RAM starting at 0x80000000.
    la sp, boot_stack_top

    # 2. Set the machine-mode trap vector.
    la t0, trap_handler
    csrw mtvec, t0

    # 4. Jump to function
    tail rust_main

.section .bss.stack
.global boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 4  # 16KB stack space
.global boot_stack_top
boot_stack_top:

.section .bss.trap_stack
.global trap_stack_lower_bound
trap_stack_lower_bound:
    .space 4096 * 4  # 16KB trap stack
.global trap_stack_top
trap_stack_top:
