/* RAM on QEMU's virt machine starts at 0x40000000 */
MEMORY {
    ram : ORIGIN = 0x40000000, LENGTH = 64M
}

PROVIDE(__el1_stack_size = 0x40000);
ENTRY(_el1_entry)

SECTIONS {
    /DISCARD/ : { *(.note*) }
}
