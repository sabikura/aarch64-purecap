INCLUDE memory.x

SECTIONS {
    .vectors : ALIGN(2048) {
        __el1_vectors_start = .;
        KEEP(*(.vectors.el1))
    } > ram

    .text : {
        *(.init.entry)
        *(.text*)
    } > ram

    .rodata : { *(.rodata*) } > ram
    .data   : { *(.data*)   } > ram

    .bss : {
        __el1_bss_start = .;
        *(.bss*)
        __el1_bss_end = .;
    } > ram

    .stack (NOLOAD) : ALIGN(16) {
        __el1_stack_start = .;
        . += 0x10000;
        __el1_stack_size = . - __el1_stack_start;
    } > ram

    /DISCARD/ : { *(.comment) *(.eh_frame) }
}
