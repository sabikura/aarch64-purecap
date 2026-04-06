#!/usr/bin/env bash

: "${CHERI_HOME:=$HOME/cheri}"

exec "$CHERI_HOME/output/morello-sdk/bin/aarch64-unknown-freebsd-objdump" "$@"
