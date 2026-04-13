//! TODO: Document this crate and usage
#![no_std]

pub use aarch64_purecap_rt_macros::entry;
pub use aarch64_purecap_rt_macros::exception;

// EL1 vector table
core::arch::global_asm!(
    r#"
    .section .vectors.el1, "ax"
    .global _el1_vectors
    // Save to stack the capability registers. For the Morello architecture, capabilities and
    // normal registers are stored in the same register file (x0 == c0[64:0])
    // c0-c18,c29,c30 are caller saved by AAPCS + (celr, spsr) in case of nesting
    .macro store_regs
        stp c0,  c1,  [csp, #-(16 * 23)]!
        stp c2,  c3,  [csp, #(16 * 2)]
        stp c4,  c5,  [csp, #(16 * 4)]
        stp c6,  c7,  [csp, #(16 * 6)]
        stp c8,  c9,  [csp, #(16 * 8)]
        stp c10, c11, [csp, #(16 * 10)]
        stp c12, c13, [csp, #(16 * 12)]
        stp c14, c15, [csp, #(16 * 14)]
        stp c16, c17, [csp, #(16 * 16)]
        str c18,      [csp, #(16 * 18)]
        stp c29, c30, [csp, #(16 * 19)]
        
        mrs c0, CELR_EL1
        mrs x1, SPSR_EL1
        str c0, [csp, #(16 * 21)]
        str x1, [csp, #(16 * 22)]
    .endm
    // Restore the registers state from the stack
    .macro restore_regs
        ldp c2,  c3,  [csp, #(16 * 2)]
        ldp c4,  c5,  [csp, #(16 * 4)]
        ldp c6,  c7,  [csp, #(16 * 6)]
        ldp c8,  c9,  [csp, #(16 * 8)]
        ldp c10, c11, [csp, #(16 * 10)]
        ldp c12, c13, [csp, #(16 * 12)]
        ldp c14, c15, [csp, #(16 * 14)]
        ldp c16, c17, [csp, #(16 * 16)]
        ldr c18,      [csp, #(16 * 18)]
        ldp c29, c30, [csp, #(16 * 19)]
        
        ldr c0, [csp, #(16 * 21)]
        ldr x1, [csp, #(16 * 22)]
        msr CELR_EL1, c0
        msr SPSR_EL1, x1

        ldp c0, c1, [csp], #(16 * 23)
    .endm
    // Exception handler for exceptions that should not occur (hehe)
    .macro invalid_entry label
    .align  7
    \label:
        b .
    .endm

    // EL1 vector table
    .align 11
    _el1_vectors:
        invalid_entry el1_sp0_sync
        invalid_entry el1_sp0_irq
        invalid_entry el1_sp0_fiq
        invalid_entry el1_sp0_serror

        .align 7
        el1_sync:
            store_regs
            mov c0, csp
            bl __aarch64_purecap_rt_el1_sync
            restore_regs

        .align 7
        el1_irq: 
            store_regs
            mov c0, csp
            bl __aarch64_purecap_rt_el1_irq
            restore_regs

        invalid_entry el1_sp1_fiq
        invalid_entry el1_sp1_serror

        .align 7
        el0_sync:
            store_regs
            bl __aarch64_purecap_rt_el0_sync
            restore_regs

        .align 7
        el0_irq: 
            store_regs
            bl __aarch64_purecap_rt_el0_irq
            restore_regs

        invalid_entry el0_a64_fiq
        invalid_entry el0_a64_serror

        invalid_entry el0_a32_sync
        invalid_entry el0_a32_irq
        invalid_entry el0_a32_fiq
        invalid_entry el0_a32_serror
    "#
);

// Configure environment before jumping to the user defined function:
// - stack pointer capability has the address set to __el1_stack_end and the bounds to
//   __el1_stack_size
// - vector table and entry point capabilities have the address set to __el1_vectors_start,
//   and __el1_entry_start, respectively, and have the bounds derived from program counter
//   capability (PCC)
core::arch::global_asm!(
    r#"
    .section .init.entry, "ax"
    .global _el1_entry
    .type _el1_entry, %function
    _el1_entry:
        // Configure the stack capability for EL1
        ldr x0, =__el1_stack_start
        cvtd c0, x0
        ldr x2, =__el1_stack_size
        // NOTE: The bounds for c1 (base=ddc_base(0),length=ddc_length(full address space)
        //       should get translated to (base=__el1_stack_start, length=__el1_stack_size)
        // NOTE: Should this be done in EL1 and just change the address here?
        scbnds c0, c0, x2
        msr CSP_EL1, c0
        
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
        cvtd c0, x0
        msr CVBAR_EL1, c0

        // Zero BSS out
        ldr x0, __el1_bss_start
        cvtd c0, x0
        ldr x1, __el1_bss_end
        cvtd c1, x1
        1:
            cmp c0, c1
            b.ge 2f
            str xzr, [c0], #8
            b 1b
        2:

        b __aarch64_purecap_rt_main 
    "#
);
