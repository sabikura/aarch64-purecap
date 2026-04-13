#!/usr/bin/env bash

: "${CHERI_HOME:=$HOME/cheri}"

# TODO: Investigate why clang failed to link here?
#
# exec "$CHERI_HOME/output/morello-sdk/bin/clang" -target aarch64-none-elf -march=morello+c64 -mabi=purecap "$@"

exec "$CHERI_HOME/output/morello-sdk/bin/ld.lld" "$@"
