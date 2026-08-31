//! Relocate global capabilities
//!
//! Since capabilities can't be created, only derived, global capabilities have to be created
//! at startup from a special capability relocation table.

/// Capability relocation table entry. This layout was retrieved from
/// `morello-llvm-project/clang/lib/Headers/cheri_init_globals.h`
#[repr(C)]
#[derive(Debug)]
struct CapReloc {
    /// `.data` slot reserved for the capability
    location: usize,
    /// Capability base
    base: usize,
    /// Capability offset
    offset: usize,
    /// Capability size
    size: usize,
    /// Capability permissions. According to the documentation from the llvm source code,
    /// for Morello this field should fit 1:1 with the scalar operand for CLRPERM:
    ///
    /// - Executable       0x8000000000013DBCULL
    /// - Read-Write Data  0x8FBEULL
    /// - Read-Only Data   0x1BFBEULL
    perms: usize,
}

/// Flag for function relocation sits in the perms field as the 63rd bit. This is
/// not a valid capability permission and it's used only to mark that the capability
/// should be configured as sentry.
const FUNCTION_RELOC_FLAG: usize = 1 << (usize::BITS - 1);

/// Initialise the global capabilities based on the compiler-generated capability relocation table
#[doc(hidden)]
#[no_mangle]
#[link_section = ".text"]
pub unsafe extern "C" fn __init_cap_relocs() {
    let start_address = cap_relocs_start();
    let end_address = cap_relocs_end();

    let table: *const CapReloc;

    unsafe {
        core::arch::asm!(
            "cvtd {cap:x}, {addr}",
            cap = out(reg) table,
            addr = in(reg) start_address,
            options(nomem, nostack, preserves_flags),
        );
    }

    let table = unsafe {
        let len = (end_address - start_address) / core::mem::size_of::<CapReloc>();
        core::slice::from_raw_parts(table, len)
    };

    for entry in table {
        // Write the capability to the location specified in the entry
        let location: *mut *const u8;
        unsafe {
            core::arch::asm!(
                "cvtd {cap:x}, {addr}",
                cap = out(reg) location,
                addr = in(reg) entry.location,
                options(nomem, nostack, preserves_flags),
            );
        }

        if entry.base == 0 {
            *location = core::ptr::null();
            continue;
        }

        let perms = entry.perms;
        let ptr = if (perms & FUNCTION_RELOC_FLAG) == FUNCTION_RELOC_FLAG {
            // FIXME: This capability is computed once here and once in the entry assembly code.
            //        Maybe make it into a global_asm macro.

            // Derive the capability from PCC, bounded to the code region.
            // Called functions run with PCC set to this capability, and they
            // reach globals through the .got, so the bounds cover
            // [__el1_code_start, __el1_code_end) and not only the function.
            let code_start = code_start();
            let code_len = code_end() - code_start;
            let mut ptr: *const u8;
            unsafe {
                core::arch::asm!(
                    "cvtp {cap:x}, {base}",
                    "scbndse {cap:x}, {cap:x}, {len}",
                    "scvalue {cap:x}, {cap:x}, {addr}",
                    cap = out(reg) ptr,
                    base = in(reg) code_start,
                    len = in(reg) code_len,
                    addr = in(reg) entry.base + entry.offset,
                    options(nomem, nostack, preserves_flags),
                );
            }

            // Set the permissions of the capability and configure it as sentry
            unsafe {
                core::arch::asm!(
                    "clrperm {cap:x}, {cap:x}, {mask}",
                    // RB = register branch
                    "seal {cap:x}, {cap:x}, rb",
                    cap = inout(reg) ptr, mask = in(reg) perms,
                    options(nomem, nostack, preserves_flags),
                );
            }

            ptr
        } else {
            // Derive the capability from DDC with the entry specified base address
            let mut ptr: *const u8;
            unsafe {
                core::arch::asm!(
                    "cvtd {cap:x}, {addr}",
                    cap = out(reg) ptr,
                    addr = in(reg) entry.base,
                    options(nomem, nostack, preserves_flags),
                );
            }

            if entry.size != 0 {
                // Set the length of the capability
                unsafe {
                    core::arch::asm!(
                        "scbnds {cap:x}, {cap:x}, {len}",
                        cap = inout(reg) ptr,
                        len = in(reg) entry.size,
                        options(nomem, nostack, preserves_flags),
                    );
                }
            }

            // Set the base and permissions of the capability
            let address = entry.base + entry.offset;
            unsafe {
                core::arch::asm!(
                    "scvalue {cap:x}, {cap:x}, {off}",
                    "clrperm {cap:x}, {cap:x}, {mask}",
                    cap = inout(reg) ptr,
                    off = in(reg) address,
                    mask = in(reg) perms,
                    options(nomem, nostack, preserves_flags),
                );
            }

            ptr
        };

        *location = ptr;
    }
}

#[inline(always)]
fn code_start() -> usize {
    let mut tmp: *mut u8;
    let start: usize;
    unsafe {
        core::arch::asm!("adrp {cap:x}, __el1_code_start", cap = out(reg) tmp, options(nomem, nostack, pure));
        core::arch::asm!("add {cap:x}, {cap:x}, :lo12:__el1_code_start", cap = inout(reg) tmp, options(nomem, nostack, pure));
        core::arch::asm!("gcvalue {val:x}, {cap:x}", val = out(reg) start, cap = in(reg) tmp, options(nomem, nostack, pure));
    }
    start
}

#[inline(always)]
fn code_end() -> usize {
    let mut tmp: *mut u8;
    let end: usize;
    unsafe {
        core::arch::asm!("adrp {cap:x}, __el1_code_end", cap = out(reg) tmp, options(nomem, nostack, pure));
        core::arch::asm!("add {cap:x}, {cap:x}, :lo12:__el1_code_end", cap = inout(reg) tmp, options(nomem, nostack, pure));
        core::arch::asm!("gcvalue {val:x}, {cap:x}", val = out(reg) end, cap = in(reg) tmp, options(nomem, nostack, pure));
    }
    end
}

#[inline(always)]
pub unsafe fn cap_relocs_start() -> usize {
    let mut tmp: *mut u8;
    let start: usize;
    unsafe {
        core::arch::asm!("adrp {cap:x}, __cap_relocs_start", cap = out(reg) tmp, options(nomem, nostack, pure));
        core::arch::asm!("add {cap:x}, {cap:x}, :lo12:__cap_relocs_start", cap = inout(reg) tmp, options(nomem, nostack, pure) );
        core::arch::asm!("gcvalue {val:x}, {cap:x}", val = out(reg) start, cap = in(reg) tmp, options(nomem, nostack, pure));
    }
    start
}

#[inline(always)]
pub unsafe fn cap_relocs_end() -> usize {
    let mut tmp: *mut u8;
    let end: usize;
    unsafe {
        core::arch::asm!("adrp {cap:x}, __cap_relocs_end", cap = out(reg) tmp, options(nomem, nostack, pure));
        core::arch::asm!("add {cap:x}, {cap:x}, :lo12:__cap_relocs_end", cap = inout(reg) tmp, options(nomem, nostack, pure));
        core::arch::asm!("gcvalue {val:x}, {cap:x}", val = out(reg) end, cap = in(reg) tmp, options(nomem, nostack, pure));
    }
    end
}
