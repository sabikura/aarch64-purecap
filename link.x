MEMORY {
    ram (rwx) : ORIGIN = 0x80000000, LENGTH = 0x7f000000
}

__el1_stack_size = 0x10000;

/* TODO: improve this */

ENTRY(_el2_drop_to_el1);

EXTERN(__aarch64_purecap_rt_main);

/* EXTERN(_el2_drop_to_el1); */

SECTIONS {
    .vectors: {
        KEEP(*(.vectors))
        __el1_vectors_start = .;
        KEEP(*(.vectors.el1))
        __el2_vectors_start = .;
    } >ram

    .text : {
        KEEP(*(.text._el2_drop_to_el1))
        __el1_entry_start = .;
        KEEP(*(.text.el1_entry))
        __el1_entry_end   = .;
        KEEP(*(.text))
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
