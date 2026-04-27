#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"

: "${CHERI_HOME:=$HOME/cheri}"

make_flags=(
    -j"$(nproc)"
    ARCH=aarch64
    E=0
    PLAT=morello
    ARM_ROTPK_LOCATION="devel_rsa"
    CREATE_KEYS=1
    GENERATE_COT=1
    MBEDTLS_DIR="$DIR/mbedtls"
    ROT_KEY="$DIR/trusted-firmware-a/plat/arm/board/common/rotpk/arm_rotprivk_rsa.pem"
    TRUSTED_BOARD_BOOT=1
    CC="$CHERI_HOME/output/morello-sdk/bin/clang"
    LD="$CHERI_HOME/output/morello-sdk/bin/ld.lld"
    CROSS_COMPILE="$CHERI_HOME/output/morello-sdk/bin/llvm-"
    TARGET_PLATFORM=fvp
    ENABLE_MORELLO_CAP=1
)

make -C "$DIR/trusted-firmware-a" "${make_flags[@]}" certtool
make -C "$DIR/trusted-firmware-a" "${make_flags[@]}" fiptool
make -C "$DIR/trusted-firmware-a" "${make_flags[@]}" all BL33="$DIR/helloworld.bin"

mkdir -p "$DIR/output"
cp "$DIR/trusted-firmware-a/build/morello/release/"*.bin "$DIR/output/"
cp "$DIR/trusted-firmware-a/build/morello/release/fdts/"*.dtb "$DIR/output/"

