# aarch64-purecap-rt

Library for a simple startup routine for pure capability application on the Aarch64 cores that have
the CHERI extension, i.e. Arm Morello architecture.

## Getting started

### Getting the compiler

TODO: Add config.toml in `./compiler`

First things first, the support for Arm Morello in `rustc` is not yet upstreamed.
This repository contains a patch for adding support for the `aarch64-unknown-none-purecap` target
triple. The "support" I added is just a config file because the current state had only FreeBSD
targets. The whole work to get rustc to work with the Morello LLVM backend was done by the
University of Kent team and is really impressive.

Clone the repository with the compiler:

```shell
git clone --depth 1 https://github.com/kent-weak-memory/rust.git $CHERI_RUST
```

Take the patch file `aarch64-unknown-none-purecap.patch` from this repository and apply it to your clone:

```shell
cd $CHERI_RUST
git apply /path/to/aarch64-purecap-rt/compiler/aarch64-unknown-none-purecap.patch
```

### Building the compiler

The instructions on building the compiler are found in `rust/README.md` in the **Building from source** chapter,
and additional helpful information is in `rust/CHERI-NOTES.md`.

> **Important:**
> One key thing you have to do is make sure you build the `core` library using: `./x.py build library` and `cargo`
using `./x.py build tools/cargo`.

### Building the crate

In order to build the crate, the Cargo config uses the scripts in `tools` for the compiler and linker.
You will need to edit the `$CHERI_HOME` and `$CHERI_RUST` variables.

> **Note:**
> You will also have to use the `cargo` version that you previously compiled in the CHERI rust repository. You'll find a helper script
in `tools/cargo.sh` that you can use.

