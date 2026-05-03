# Getting started

## Toolchain

First things first, the support for Arm Morello in the Rust compiler is not yet upstreamed.
This repository contains a patch for adding support for the `aarch64-unknown-none-purecap` target
triple. The "support" I added is just a config file because the current state had only FreeBSD
targets. The whole work to get rustc to work with the Morello LLVM backend was done by the
University of Kent team and is really impressive ^_^.

Clone the Morello `rust` fork:

```shell
git clone --depth 1 https://github.com/kent-weak-memory/rust.git $CHERI_RUST
```

> Note:
> The Morello SDK should be built according to the rust fork's instructions, since a specific LLVM
> version has to be used. The resulting SDK is expected at `$CHERI_HOME` (default `$HOME/cheri`).

Take the patch file `aarch64-unknown-none-purecap.patch` from this repository and apply it to your clone:

```shell
cd $CHERI_RUST
git apply /path/to/aarch64-purecap/toolchain/compiler/aarch64-unknown-none-purecap.patch
```

### Building the compiler

The instructions on building the compiler are found in `rust/README.md` in the **Building from source** chapter,
and additional helpful information is in `rust/CHERI-NOTES.md`. For the `config.toml` file,
make sure that the `aarch64-unknown-none-purecap` target is added:

```toml
[build]
target = ["x86_64-unknown-linux-gnu", "aarch64-unknown-none-purecap"]

# ...

[target.aarch64-unknown-none-purecap]
llvm-config = "/path/to/cheri/output/morello-sdk/bin/llvm-config"
cc = "/path/to/rust/clang-morello.sh"
cxx = "/path/to/rust/clang++-morello.sh"
linker = "/path/to/rust/clang-morello.sh"
ar = "/path/to/cheri/output/morello-sdk/bin/ar"
ranlib = "/path/to/cheri/output/morello-sdk/bin/ranlib"
```

> **Important:**
> One key thing you have to do is make sure you build the `core` library using: `./x.py build library` and `cargo`
using `./x.py build tools/cargo`.

## Using the toolchain

You will need to set the `$CHERI_HOME` and `$CHERI_RUST` environment variables, defaulting to `$HOME/cheri` and `$HOME/rust`, respectively.
Although the `xtask` tool can be used with the upstream cargo, it will use the `toolchain/` scripts for the inner work. Check if everything
was configured properly by trying to build the `aarch64-purecap-rt` crate with:

```shell
cargo xtask build --release
```

> **Note:**
> Some error logs from the compiler relating to the assembly might appear when running the build command,
> but the instructions look correctly generated after inspecting the `objdump` output of the application binary.

### Morello Fixed Virtual Platform (FVP)

For running the example, you will need the `Morello Fixed Virtual Platform (FVP)` tool to simulate the Morello SoC. The run script in `examples/fvp`
assumes that it was installed with `cheribuild.py` and it's found in `$CHERI_HOME`.

