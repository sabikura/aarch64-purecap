# `helloworld-morello-fvp`

A minimal example that prints a short message over UART on the Morello FVP using the `aarch64-purecap-rt` runtime.
The program boots in EL2 hybrid mode and drops into pure-capability EL1 to print the message.

> Note:
> See the workspace `README.md` for build/run instructions (`cargo xtask {setup,fip,fvp,clean} helloworld-morello-fvp`)
> and `QUICKSTART.md` for toolchain setup.
