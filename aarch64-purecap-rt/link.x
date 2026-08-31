INCLUDE memory.x

SECTIONS {
    /* PCC covered region: .text (with literal pools), .vectors and .got.
       Everything the CPU fetches or reads PC-relative must sit between
       __el1_code_start and __el1_code_end. Data is reached through
       capabilities loaded from the .got, so it stays outside. */
    .text : {
        __el1_code_start = .;
        KEEP(*(.text._el1_entry))
        *(.text*)
    } > ram

    .vectors : ALIGN(2048) {
        __el1_vectors_start = .;
        KEEP(*(.vectors.el1))
        . = ALIGN(0x800);
    } > ram

    .got : {
        *(.got)
        *(.got.plt)
    } > ram

    /* Capability slots for internal globals. The compiler reads them
       PC-relative like the .got, so they belong to the PCC region. They are
       only written by __init_cap_relocs, through DDC, before DDC is nulled. */
    .data.rel.ro : {
        *(.data.rel.ro*)
    } > ram

    /* Also read PC-relative (constant pools), so it stays inside the PCC
       region. PCC has no Store permission, only writable memory is outside. */
    .rodata : {
        *(.rodata*)
        . = ALIGN(4096);
        __el1_code_end = .;
    } > ram

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

    /* Start of the heap arena handed out by the heap grant. The length is the
       const generic of the Heap grant, so only the start address is set here. */
    PROVIDE(__el1_heap_start = __el1_stack_end);

    /DISCARD/ : { *(.comment) *(.eh_frame) }
}
