MEMORY {
    ram (rwx) : ORIGIN = 0x80000000, LENGTH = 0x7f000000
}

/* TODO: improve this */
ENTRY(_entry_el2);
EXTERN(_stack_setup);
EXTERN(_init_stack);

SECTIONS {
    .text : {
        KEEP(*(.text.*))
	} >ram
}
