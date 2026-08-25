# Getting started

## Toolchain

First things first, the support for Arm Morello in the Rust compiler is not yet upstreamed.
This repository contains a patch for adding support for the `aarch64-unknown-none-purecap` target
triple. The "support" I added is just a config file because the current state had only FreeBSD
targets. The whole work to get rustc to work with the Morello LLVM backend was done by the
University of Kent team and is really impressive ^_^.

I've built a CLI tool that simplifies the process of setting the whole toolchain:
[crability](https://github.com/sabikura/crability.git). After setting up your environment,
you can just call:

```bash
crability cargo check # for the whole workspace
crability cargo check -p aarch64-purecap-rt --target aarch64-unknown-none-purecap # the rt crate
```
