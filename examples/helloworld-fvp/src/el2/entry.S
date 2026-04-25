// Entry point for dropping from EL2 to EL1. This runtime is used alongside Arm Trusted Firmware that
// starts BL33 in the non-secure world at EL2.
//
// System registers are configured so that EL1 starts in Aarch64 full capability mode,
// doesn't trap to EL2 on WFI, virtual timer access, floating point or SIMD instructions.

.section .text._el2_drop_to_el1
.global _el2_drop_to_el1 
.type _el2_drop_to_el1, %function
.extern __el1_entry_start
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

    // Configure SCTLR_EL1: disable MMU and caches but preserve Morello-specific bits
    mrs x0, SCTLR_EL1
    bic x0, x0, #(1 << 0)    // M:  MMU off
    bic x0, x0, #(1 << 2)    // C:  Data cache off
    bic x0, x0, #(1 << 12)   // I:  Instruction cache off
    msr SCTLR_EL1, x0

    // Enable capability instructions at EL1/EL0 (CPACR_EL1.CEN = 0b11)
    // Without this, BX #4 and other capability instructions fault at EL1.
    mrs x0, CPACR_EL1
    orr x0, x0, #(3 << 18)   // CEN = 0b11: no trapping
    msr CPACR_EL1, x0

    // Configure the status register
    // ARM DDI 0606; 3.2.41
    mov x0, #0x3C5          // Mask D,A,I,F exceptions and return to EL1h stack pointer
    orr x0, x0, #(1 << 26)  // EL1 in full capability mode (C64)
    msr SPSR_EL2, x0

    // Configure the Default Data Capability (DDC) for EL1 to full system access
    // NOTE: I think this is useless;
    //       ddc from all levels seem to have the same reset value
    // mrs c0, DDC_EL2
    // msr DDC_EL1, c0

    // DIAGNOSTIC: eret to test stub (A64 mode, within PCC bounds).
    // adr x0, _el1_test_stub
    ldr x0, =__el1_entry_start
    cvtp c0, x0
    msr CELR_EL2, c0

    // Give EL1 the same DDC authority as EL2
    mrs     c0, DDC
    msr     DDC_EL1, c0

    isb
    eret
