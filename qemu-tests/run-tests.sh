#!/usr/bin/env bash

# Build and run every QEMU test binary. A test passes when QEMU exits 0.
# Uses the crability cargo, the host cargo passes flags the forked rustc
# does not know.

set -u
cd "$(dirname "$0")"

fail=0
for bin in bounded fault_ddc fault_pcc; do
    echo "=== $bin ==="
    if ../toolchain/cargo.sh run --release --bin "$bin"; then
        echo "=== $bin: pass ==="
    else
        echo "=== $bin: fail (exit $?) ==="
        fail=1
    fi
done

exit "$fail"
