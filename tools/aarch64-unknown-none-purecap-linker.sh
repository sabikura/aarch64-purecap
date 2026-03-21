#!/usr/bin/env bash

: "${CHERI_HOME:=$HOME/cheri}"

exec "$CHERI_HOME/output/morello-sdk/bin/clang --sysroot $CHERI_HOME/output/rootfs-morello-purecap/ -target aarch64-unknown-none -march=morello+c64 -mabi=purecap" "$@"
