// Entry point for dropping from EL2 to EL1. This runtime is used alongside Arm Trusted Firmware that
// starts BL33 in the non-secure world at EL2.

.section .text._el2_drop_to_el1
.global _el2_drop_to_el1 
.type _el2_drop_to_el1, %function
_el2_drop_to_el1:
    // Disable traps for co-processor access
    // ARM DDI 0487; D24.2.77
    msr HSTR_EL2, xzr

    // Disable traps for advanced functionalities like FP or CHERI
    // ARM DDI 0606; 3.2.11
    mrs x0, CPTR_EL2
    bic x0, x0, #(1 << 31)               // TCPAC
    bic x0, x0, #(1 << 28)               // TTA
    bic x0, x0, #(1 << 10)               // TFP
    bic x0, x0, #(1 << 9)                // TC
    bic x0, x0, #(1 << 8)                // TZ
    orr x0, x0, #((1 << 20) | (1 << 21)) // FPEN
    orr x0, x0, #((1 << 19) | (1 << 18)) // CEN
    orr x0, x0, #((1 << 17) | (1 << 16)) // ZEN
    msr CPTR_EL2, x0

    // Don't know if this will bite later, but allow EL1 to execute
    // privileged capability creating instructions
    // ARM DDI 0606; 3.2.7
    mrs x0, CHCR_EL2
    bic x0, x0, 1 // SETTAG
    msr CHCR_EL2, x0

    // More configuration
    // NOTE: This is a BIG register, might need to look at each bit in the future (?)
    // ARM DDI 0487; D24.2.62
    mrs x0, HCR_EL2
    bic x0, x0, #(1 << 13) // TWI: Don't trap WFI instructions
    orr x0, x0, #(1 << 31) // RW:  EL1 runs in Aarch64
    msr HCR_EL2, x0

    // Configure access to timers
    // ARM DDI 0487; D24.10.2
    mrs x0, CNTHCTL_EL2
    orr x0, x0, #((1 << 10) | (1 << 9))  // EL1PTEN | EL1PCTEN: don't trap timer access
    msr CNTHCTL_EL2, x0 
    // ARM DDI 0487; D24.10.30
    msr CNTVOFF_EL2, xzr // Virtual offset for the timer is 0

    // TODO: Configure SCTLR_EL1 (not understanding the fields yet)

    // Configure the status register
    // ARM DDI 0606; 3.2.41
    mov x0, #0x3C5          // Mask D,A,I,F exceptions and return to EL1h stack pointer
    orr x0, x0, #(1 << 26)  // EL1 in full capability mode (C64)
    msr SPSR_EL2, x0

    // Configure the Default Data Capability (DDC) for EL1 to full system access
    // NOTE: I think this is useless;
    //       ddc from all levels seem to have the same reset value
    mrs c0, DDC_EL2
    msr DDC_EL1, c0

    // Configure the entry point capability for EL1
    // ARM DDI 0606; 3.2.22
    ldr x0, =__el1_entry_start
    adr x1, .
    scvalue c1, c1, x0
    msr CELR_EL2, c1

    // Configure the stack capability for EL1
    ldr x0, =__el1_stack_start
    scvalue c1, c0, x0
    ldr x2, =__el1_stack_size
    // NOTE: The bounds for c1 (base=ddc_base(0),length=ddc_length(full address space)
    //       should get translated to (base=__el1_stack_start, length=__el1_stack_size)
    // NOTE: Should this be done in EL1 and just change the address here?
    scbnds c1, c1, x2
    msr CSP_EL1, c1
    
    // Configure SPSel to be 1 (exceptions use SP_EL1, not SP_EL0)
    // ARM DDI 0487; C5.2.18
    msr SPSel, #1

    // Configure EL1 vector table (CVBAR_EL1)
    // ARM DDI 0606; 3.2.48
    //
    // NOTE: The bounds for the CVBAR_EL1 are derived from the PCC with the base set to
    //       the vector table start. Deriving from the PCC is done by first loading the current
    //       program counter into c2, and modifying the address with scvalue, keeping the PCC
    //       bounds unmodified.
    ldr x0, =__el1_vectors_start
    scvalue c1, c1, x0
    msr CVBAR_EL1, c1

    // Zero BSS out
    adr c1, __bss_start
    adr c2, __bss_end
    1:
        cmp c1, c2
        b.ge 2f
        str xzr, [c1], #8
        b 1b
    2:

    isb
    eret
