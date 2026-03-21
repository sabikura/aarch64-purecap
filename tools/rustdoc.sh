#!/usr/bin/env bash

: "${CHERI_RUST:=$HOME/rust}"

exec "$CHERI_RUST/build/x86_64-unknown-linux-gnu/stage0/bin/rustdoc" \
    --sysroot="$CHERI_RUST/build/x86_64-unknown-linux-gnu/stage1" "$@"
