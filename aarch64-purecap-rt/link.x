INCLUDE memory.x

SECTIONS {
    .text : {
        KEEP(*(.text._el1_entry))
        *(.text*)
    } > ram

    .vectors : ALIGN(2048) {
        __el1_vectors_start = .;
        KEEP(*(.vectors.el1))
        . = ALIGN(0x800);
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
        . += __el1_stack_size;
        __el1_stack_end = .;
    } > ram

    /DISCARD/ : { *(.comment) *(.eh_frame) }
}
