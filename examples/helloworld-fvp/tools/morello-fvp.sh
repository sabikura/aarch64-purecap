#!/usr/bin/env bash

: "${CHERI_HOME:=$HOME/cheri}"

exec "$CHERI_HOME/output/morello-sdk/FVP_Morello/models/Linux64_GCC-6.4/FVP_Morello" \
    "$@"
