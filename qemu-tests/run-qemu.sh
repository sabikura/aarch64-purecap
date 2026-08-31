#!/usr/bin/env bash

# Cargo runner: boots the ELF on CHERI QEMU's virt machine with a Morello CPU.
# The exit code is the semihosting exit code of the test binary.

: "${CRABILITY_BIN:=$HOME/.crability/bin}"

exec timeout 60 "$CRABILITY_BIN/qemu-system-morello" \
    -M virt \
    -cpu morello \
    -nographic \
    -semihosting \
    -kernel "$1"
