OUTPUT_ARCH("riscv")
ENTRY(_start)

BASE_ADDRESS = 0x80000000;

PHDRS
{
    text PT_LOAD FLAGS(5);
    data PT_LOAD FLAGS(6);
}

SECTIONS
{
    /* QEMU RISC-V Virt machine jumps right to 0x80000000 */
    . = BASE_ADDRESS;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    } :text

    .rodata : {
        *(.rodata .rodata.*)
    } :text

    .data : {
        *(.data .data.*)
    } :data

    .bss : {
        *(.bss .bss.*)
    } :data

    
    /* 1 MiB of memory, like god intended */
    . = ALIGN(16);
    _heap_start = .;
    . = . + 0x100000;
    _heap_end = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}
