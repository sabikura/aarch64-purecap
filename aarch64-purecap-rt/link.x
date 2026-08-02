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

    .__cap_relocs : {
        __cap_relocs_start = .;
        KEEP(*(__cap_relocs))
        KEEP(*(.__cap_relocs))
        __cap_relocs_end = .;
    } > ram

    .data   : { *(.data*)   } > ram

    .bss (NOLOAD) : ALIGN(16) {
        __el1_bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        . = ALIGN(16);
        __el1_bss_end = .;
    } > ram

    .stack (NOLOAD) : ALIGN(4096) {
        __el1_stack_start = .;
        . += __el1_stack_size;
        . = ALIGN(16);
        __el1_stack_end = .;
    } > ram

    /DISCARD/ : { *(.comment) *(.eh_frame) }
}
