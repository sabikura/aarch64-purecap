//! Library for a simple startup routine for CHERI Aarch64 (ARM Morello architecture) pure capability
//! bare-metal applications. This crate provides the memory layout of the program, provides setup
//! code that configures the stack and the vector table, and jumps to the entry point defined by the
//! user.
//!
//! # Usage
//!
//! This crate can't be built with upstream Rust. Check the `QUICKSTART.md` to build the
//! Morello-compatible toolchain.
//!
//! This crate expects that the user provides a `memory.x` linker script that defines the `ram`
//! memory region and the `__el1_stack_size` symbol.
//!
//! This crate currently offers the user the possibility to provide custom handlers for
//! EL1/EL0 synchronous or asynchronous (irq) exceptions. Other types will be added in the future.
//!
//! ## Example
//!
//! ### `memory.x`
//!
//! ```
//! MEMORY {
//!     ram : ORIGIN = 0x80000800, LENGTH = 64M
//! }
//! PROVIDE(__el1_stack_size = 0x10000);
//! ENTRY(_el1_entry)
//! ```
//!
//! ### `main.rs`
//!
//! ```rust,ignore
//! #![no_std]
//! #![no_main]
//!
//! use aarch64_purecap_rt::{entry, exception};
//!
//! #[entry]
//! fn main() -> ! {
//!     loop {}
//! }
//!
//! #[panic_handler]
//! fn panic(_info: &core::panic::PanicInfo) -> ! {
//!     loop {}
//! }
//!
//! // Optional: Define a custom exception handler
//! #[exception(EL1, Sync)]
//! fn el1_sync_handler() {}
//! ```

#![no_std]
#![feature(cfg_target_abi)]

#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
pub use aarch64_purecap_rt_macros::entry;

#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
pub use aarch64_purecap_rt_macros::exception;

#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
mod cap_relocs;

mod grant;

// If the platform boots in A64 mode, this enables first the capability intructions
// then toggles the instruction set to C64 using the `bx 4` instruction
// (DDI0606 Section 4.4.20)
#[cfg(feature = "hybrid")]
#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
macro_rules! capability_shim {
    () => {
        r#"
            .code a64
            mrs x0, CPACR_EL1
            // Enable floating point for EL1/0 (issue had in qemu)
            orr x0, x0, #(3 << 20)
            // Enable capability instructions for EL1/0
            orr x0, x0, #(3 << 18)
            msr CPACR_EL1, x0
            isb
            // Toggle the instruction set to C64 and enter purecap
            bx #4
            .code c64
        "#
    };
}

// Does nothing for platforms that boot into C64 mode
#[cfg(not(feature = "hybrid"))]
#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
macro_rules! capability_shim {
    () => {
        ""
    };
}

// Configure environment before jumping to the user defined function:
// - stack pointer capability has the address set to __el1_stack_end and the bounds to
//   __el1_stack_size
// - vector table and entry point capabilities have the address set to __el1_vectors_start,
//   and __el1_entry_start, respectively, and have the bounds derived from program counter
//   capability (PCC)
#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
core::arch::global_asm!(
    r#"
    .arch morello+c64
    .section .text._el1_entry, "ax"
    .global _el1_entry
    .type _el1_entry, %function
    _el1_entry: 
    "#,
    capability_shim!(),
    r#"
        // Configure SPSel to be 1 (exceptions use SP_EL1, not SP_EL0)
        // ARM DDI 0487; C5.2.18
        msr SPSel, #1

        // Configure the stack capability for EL1

        // Compute start address into x0
        adrp    c0, __el1_stack_start           // Page address
        add     c0, c0, :lo12:__el1_stack_start // Page offset
        gcvalue x0, c0

        // Compute end address into x1
        adrp    c1, __el1_stack_end
        add     c1, c1, :lo12:__el1_stack_end
        gcvalue x1, c1

        // Compute the size into x2
        sub x2, x1, x0

        // x3 is the mask for the needed alignment so that a capability of length `x2` is
        // representable
        //
        // start = x0
        // end   = x1
        // size  = x2
        // ~(alignment - 1) = x3
        //
        // The following snippets aligns the start address of the stack to be >= initial address
        // specified in the linker script
        //
        // new_start = (start + alignment - 1) & ~(alignment - 1)
        // new_end   = end & ~(alignment - 1)
        // new_len   = new_end - new_start
        rrmask x3, x2
        mvn x4, x3
        add     x0, x0, x4
        and     x0, x0, x3
        and     x1, x1, x3
        sub     x2, x1, x0

        cvtd    c0, x0
        scbndse c0, c0, x2
        add     c0, c0, x2
        mov     csp, c0

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
        ldr x0, =__el1_bss_start
        cvtd c0, x0
        ldr x1, =__el1_bss_end
        cvtd c1, x1
        1:
            cmp c0, c1
            b.ge 2f
            str xzr, [c0], #8
            b 1b
        2:

        bl __init_cap_relocs
        isb
        dsb sy
        b __aarch64_purecap_rt_main
    "#
);

// Default exception handler
#[no_mangle]
pub unsafe extern "C" fn __aarch64_purecap_rt_default_handler() -> ! {
    loop {}
}

// Configure the exception handler symbols as weak links to the default handler.
#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
core::arch::global_asm!(
    r#"
    .weak __aarch64_purecap_rt_el0_sync
    .set __aarch64_purecap_rt_el0_sync, __aarch64_purecap_rt_default_handler 

    .weak __aarch64_purecap_rt_el1_sync
    .set __aarch64_purecap_rt_el1_sync, __aarch64_purecap_rt_default_handler 

    .weak __aarch64_purecap_rt_el1_irq 
    .set __aarch64_purecap_rt_el1_irq, __aarch64_purecap_rt_default_handler

    .weak __aarch64_purecap_rt_el0_irq 
    .set __aarch64_purecap_rt_el0_irq, __aarch64_purecap_rt_default_handler
    "#
);

// Configure the EL1 vector table.
#[cfg(all(target_arch = "aarch64", target_abi = "purecap"))]
core::arch::global_asm!(
    r#"
    .arch morello+c64
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
