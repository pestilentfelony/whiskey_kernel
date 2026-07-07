# Trap Entrypoint

.section .text
.align 2
.global trap_handler
trap_handler:
    addi sp, sp, -8 # Save space for registers
    sd t0, 0(sp) # Save t0
    sd t1, 8(sp) # Save t1 (Ensure no data loss)

    csrr t0, mcause # Read the cause of the trap into t0
    csrr t1, mepc   # Read the program counter at the time of the trap into t1

    # 3. Handle the trap event


    # 4. Restore saved registers
    ld t1, 8(sp) # Restore t1
    ld t0, 0(sp) # Restore t0
    addi sp, sp, 8 # Restore stack pointer

    mret # Use sret for stvec/Supervisor mode