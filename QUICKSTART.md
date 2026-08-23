# Getting started

## Toolchain

First things first, the support for Arm Morello in the Rust compiler is not yet upstreamed.
This repository contains a patch for adding support for the `aarch64-unknown-none-purecap` target
triple. The "support" I added is just a config file because the current state had only FreeBSD
targets. The whole work to get rustc to work with the Morello LLVM backend was done by the
University of Kent team and is really impressive ^_^.

TODO: Add crability info here
