#![no_std]

pub use aarch64_purecap_rt_macros::entry;

// Sets up the stacks
core::arch::global_asm!(
    r#"
    .section .text._stack_setup
    .global _stack_setup
    .type _stack_setup, %function
    _stack_setup:
        mov x0, xzr // PLACEHOLDER
    "#
);

// EL2 entry code (it sets up the system registers and then jumps to EL1)
core::arch::global_asm!(
    r#"
    .section .text._entry_el2
    .global _entry_el2
    .type _entry_el2, %function
    _entry_el2:
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
        // More configuration..
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
        mrs c0, DDC_EL2
        msr DDC_EL1, c0

        // TODO: Configure entry point for EL1 (CELR_EL2)
        // ldr x0, =el1_entry
        // scvalue c1, c0, x0          // derive cap from DDC
        // msr CELR_EL2, c1

        // TODO: Configure stack for EL1 (CSP_EL1)
        // ldr x0, =el1_stack_top
        // scvalue c1, c0, x0
        // ldr x2, =el1_stack_size
        // scbnds c1, c1, x2
        // msr CSP_EL1, c1

        // TODO: Configure EL1 vector table (CVBAR_EL1)
        // 
        // ldr x0, =el1_vectors
        // scvalue c1, c0, x0          // derive from DDC
        // mov x2, #0x800              // vector table size
        // scbnds c1, c1, x2           // restrict bounds
        // msr CVBAR_EL1, c1
        isb
        eret
    "#
);

core::arch::global_asm!(
    r#"
    .section .text._init_stack
    .global _init_stack
    .type _init_stack, %function
    _init_stack:
        mov x0, xzr // PLACEHOLDER
    "#
);

// EL1 entry code (sets up the vector table (I guess) and then jumps to user defined function
core::arch::global_asm!(
    r#"
    // TODO: ?
    b __aarch64_purecap_rt_main
    "#
);
