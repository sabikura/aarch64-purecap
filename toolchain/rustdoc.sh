#!/usr/bin/env bash

: "${CRABILITY_BIN:=$HOME/.crability/bin}"

exec "$CRABILITY_BIN/rustdoc" "$@"
