# Changelog

## [Unreleased]

### Features

- *(rt)* Add EL1 startup routine (`_el1_entry`) that configures the system registers and jumps to the user defined entry point
- *(rt)* Set up the EL1 stack capability and zero bss
- *(rt)* Add EL1 vector table (`CVBAR_EL1`) that saves and restores the caller-saved capability registers, `CELR_EL1` and `SPSR_EL1` around the handlers
- *(rt)* Add weak default handlers for EL1/EL0 synchronous and IRQ exceptions
- *(rt)* Provide the `link.x` linker script, which includes the user's `memory.x` (`ram` region and `__el1_stack_size`)
- *(rt)* Add cap_relocs: global capabilities are initialised at startup from the `__cap_relocs` table (function pointers as sentries derived from PCC, data derived from DDC)
- *(rt)* Add hybrid feature for qemu (boots in A64 EL1)
- *(rt)* Restrict PCC to the code region (`__el1_code_start` to `__el1_code_end`) with reduced permissions, and enter `main` through a sealed entry capability
- *(rt)* Null DDC before entering the user defined entry point
- *(rt)* Re-export the `entry` and `exception` macros
- *(macros)* Add `entry` attribute macro for the entry point (`fn(Grant) -> !`)
- *(macros)* Add `exception` attribute macro for custom `Sync`/`Irq` handlers at `El0`/`El1`
- Added grants (unstable, still needs testing)
- *(macros)* Add `grant(...)` argument for `entry`, which generates a `Grant` struct that can't be constructed by the application
- *(rt)* Add `Mmio<ADDRESS, T>` grant, handed over as a `*mut T` bounded to `T` with load/store permissions
- *(rt)* Add `Heap<SIZE>` grant, handed over as a `*mut u8` bounded to `SIZE` starting at `__el1_heap_base` (defaults to the end of the stack)
- *(rt)* Add `SealRange<START, END>` grant (accepted by `entry`, the sealing capability is not handed over yet)
- *(rt)* Add `bump-alloc` feature with a `BumpAllocator` global allocator to be paired with the heap grant (wip)

### Bug Fixes

- *(rt)* Fix linker script & stack alignment issue
- No more error logs from the global_asm
- Warnings and errors in the grants implementation

### Refactor

- *(all)* Now its a workspace (`aarch64-purecap-rt` and `aarch64-purecap-rt-macros`)
- Remove the examples and xtask (examples live in the `aarch64-purecap-examples` repository)
- Remove the rustc patch (the toolchain is set up with `crability`)
- Remove the cpu utils crate (now lives in `cheri-rs`)

### Documentation

- Add `QUICKSTART.md` for setting up the toolchain
- Update README

### Testing

- Add `trybuild` UI tests for the `entry` macro and grants

### Miscellaneous Tasks

- Add toolchain wrapper scripts and the `check-rt`/`build-rt` cargo aliases
- Add workflow (rustfmt, purecap check and tests in the `crability-ci` container)
