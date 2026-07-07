# Trap Entrypoint

.section .text
.align 2
.global trap_handler
trap_handler:
    # Switch to a dedicated trap stack using mscratch
    csrrw t6, mscratch, sp    # save old mscratch in t6, store old sp into mscratch
    la sp, trap_stack_top     # set sp to top of trap stack

    # Create a stack frame and save caller state
    addi sp, sp, -256
    sd ra, 248(sp)
    sd t0, 240(sp)
    sd t1, 232(sp)
    sd t2, 224(sp)
    sd t3, 216(sp)
    sd t4, 208(sp)
    sd t5, 200(sp)
    sd t6, 192(sp)
    sd a0, 184(sp)
    sd a1, 176(sp)
    sd a2, 168(sp)
    sd a3, 160(sp)
    sd a4, 152(sp)
    sd a5, 144(sp)
    sd a6, 136(sp)
    sd a7, 128(sp)
    sd s0, 120(sp)
    sd s1, 112(sp)
    sd s2, 104(sp)
    sd s3, 96(sp)
    sd s4, 88(sp)
    sd s5, 80(sp)
    sd s6, 72(sp)
    sd s7, 64(sp)
    sd s8, 56(sp)
    sd s9, 48(sp)
    sd s10, 40(sp)
    sd s11, 32(sp)
    sd gp, 24(sp)
    sd tp, 16(sp)

    # Read trap CSRs for handler logic
    csrr t0, mcause   # trap cause (interrupt bit + exception code)
    csrr t1, mepc     # program counter at time of trap
    csrr t2, mtval    # faulting address

    # Handle the trap event

    # Check interrupt bit (bit 63) in mcause
    srli t4, t0, 63    # Shift right logical to get the interrupt bit
    bnez t4, handle_interrupt  # If interrupt bit is set, branch
    # Exception path: fall through to exception handler
    j handle_exception

    handle_interrupt:
        # Handle machine-mode interrupts.
        # t0 = mcause, t1 = mepc, t2 = mtval
        andi t3, t0, 0x7ff      # extract interrupt cause code
        li t4, 7                # machine timer interrupt
        beq t3, t4, timer_interrupt
        li t4, 11               # machine external interrupt
        beq t3, t4, external_interrupt
        j unknown_interrupt

    timer_interrupt:
        # Call the Rust timer interrupt handler.
        call handle_timer_interrupt
        j restore_and_return

    external_interrupt:
        # No external interrupt controller is handled yet
        j restore_and_return

    unknown_interrupt:
        # Unknown interrupt; resume to avoid deadlock
        j restore_and_return

    handle_exception:
        # Handle synchronous exceptions.
        # t0 = mcause, t1 = mepc, t2 = mtval
        andi t3, t0, 0x7ff      # extract exception code
        li t4, 3                # breakpoint
        beq t3, t4, exception_breakpoint
        li t4, 2                # illegal instruction
        beq t3, t4, exception_illegal_instruction
        li t4, 5                # load access fault
        beq t3, t4, exception_load_access
        li t4, 7                # store/AMO access fault
        beq t3, t4, exception_store_access
        j unknown_exception

    exception_breakpoint:
        addi t1, t1, 4
        csrw mepc, t1
        j restore_and_return

    exception_illegal_instruction:
        # Halt here on illegal instruction.
        j trap_halt

    exception_load_access:
        # Faulting load address is in mtval.
        j trap_halt

    exception_store_access:
        # Faulting store address is in mtval.
        j trap_halt

    unknown_exception:
        # For any unhandled exception, halt for debugging.
        j trap_halt

    trap_halt:
        wfi
        j trap_halt

    # Restore saved registers and return
    restore_and_return:
    ld tp, 16(sp)
    ld gp, 24(sp)
    ld s11, 32(sp)
    ld s10, 40(sp)
    ld s9, 48(sp)
    ld s8, 56(sp)
    ld s7, 64(sp)
    ld s6, 72(sp)
    ld s5, 80(sp)
    ld s4, 88(sp)
    ld s3, 96(sp)
    ld s2, 104(sp)
    ld s1, 112(sp)
    ld s0, 120(sp)
    ld a7, 128(sp)
    ld a6, 136(sp)
    ld a5, 144(sp)
    ld a4, 152(sp)
    ld a3, 160(sp)
    ld a2, 168(sp)
    ld a1, 176(sp)
    ld a0, 184(sp)
    ld t6, 192(sp)
    ld t5, 200(sp)
    ld t4, 208(sp)
    ld t3, 216(sp)
    ld t2, 224(sp)
    ld t1, 232(sp)
    ld t0, 240(sp)
    ld ra, 248(sp)
    addi sp, sp, 256

    # restore original sp from mscratch and clear mscratch
    csrrw sp, mscratch, x0
    # restore previous mscratch value (saved in t6) so other harts/traps
    csrw mscratch, t6

    mret
