# aarch64-purecap-rt

Library for a simple startup routine for CHERI Aarch64 (ARM Morello architecture) pure capability bare-metal applications.

## Getting started

See [QUICKSTART.md](./QUICKSTART.md#getting-started) for how to build the Rust fork and the Morello SDK.

This project uses [`xtask`](https://github.com/matklad/cargo-xtask) for automation. The `xtask` command wraps the project's build, firmware-packaging,
and FVP commands so they share one toolchain and one set of paths. Run from the workspace root:

```
cargo xtask <command> [args]
```

Commands:

- `setup` - initializes git submodules and builds the TF-A Morello fork (`examples/fvp/maketfa.sh`)
- `build [cargo-args]` - builds `aarch64-purecap-rt` via the CHERI toolchain
- `check [cargo-args]` - checks`aarch64-purecap-rt`
- `build-example <name>` - builds `examples/<name>` in release mode
- `fip <name>` - builds `<name>` example, then packages a Firmware Image Package (FIP)
  with the application as BL33 (requires that `setup` was called) at `examples/fvp/output/<name>/fip.bin`
- `fvp <name> [--clean]` - boots `<name>` example on the Morello FVP. Rebuilds the
  firmware image if `fip.bin` is missing or `--clean` is passed.
- `help` - prints usage

Example workflow:

```
cargo xtask setup
cargo xtask fip helloworld-morello-fvp
cargo xtask fvp helloworld-morello-fvp --clean
```

While `cargo xtask fvp <name>` is running, the FVP exposes the AP UART over
telnet. Attach to the console from another terminal with:

```
telnet localhost 5003
```

