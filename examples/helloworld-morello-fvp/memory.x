MEMORY {
    el2 : ORIGIN = 0xE0000000, LENGTH = 2K
    ram : ORIGIN = 0xE0000800, LENGTH = 64M - 2K
}

PROVIDE(__el1_stack_size = 0x4000);
ENTRY(_el1_entry)

SECTIONS {
    .el2_entry : {
        KEEP(*(.el2_entry))
        . = ALIGN(0x800);
    } > el2

    /DISCARD/ : { *(.comment) *(.note.*) *(.eh_frame) }
}
