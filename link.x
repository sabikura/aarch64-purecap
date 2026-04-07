MEMORY {
    ram (rwx) : ORIGIN = 0x80000000, LENGTH = 0x7f000000
}

__el1_stack_size = 0x10000;

/* TODO: improve this */
ENTRY(_el2_drop_to_el1);
EXTERN(_stack_setup);
EXTERN(_init_stack);

SECTIONS {
    .text : {
        KEEP(*(.text._el2_drop_to_el1))
        __el1_entry_start = .;
        KEEP(*(.text._el1_entry))
	} >ram

    .rodata : {
        *(.rodata*)
    } >ram

    .data : {
        __el1_data_start = .;
        *(.data*)
        __el1_data_end = .;
    } >ram

    .bss (NOLOAD) : {
        __el1_bss_start = .;
        *(.bss*)
        *(COMMON)
        __el1_bss_end = .;
    } >ram

    __el1_stack_end = .;
    . = . + __el1_stack_size;
    __el1_stack_start = .;
}
