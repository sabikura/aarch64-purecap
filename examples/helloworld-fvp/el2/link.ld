MEMORY {
    ram : ORIGIN = 0x80000000, LENGTH = 16K
}

PROVIDE(__el1_entry_start = 0x80005000);
ENTRY(_el2_drop_to_el1);

SECTIONS {
    .text : {
        KEEP(*(.text._el2_drop_to_el1))
    } > ram

    /DISCARD/ : { *(.comment) *(.eh_frame) }
}
