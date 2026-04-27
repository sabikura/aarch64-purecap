#!/usr/bin/env bash
set -euo pipefail

if [ $# -lt 2 ]; then
    echo "usage: $0 <path-to-bin> <path-to-output>" >&2
    exit 1
fi

BIN_PATH="$1"
OUT_PATH="$2"

# Sanity check the paths exist
if [ ! -e "$BIN_PATH" ]; then
    echo "error: $BIN_PATH does not exist" >&2
    exit 1
fi

# Resolve to absolute path (docker --volume requires absolute paths)
BIN_PATH="$(realpath "$BIN_PATH")"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"

mkdir -p $OUT_PATH
OUT_PATH="$(realpath "$OUT_PATH")"	
common_flags=(
    --tb-fw "$DIR/output/bl2.bin"
    --soc-fw "$DIR/output/bl31.bin" 
    --nt-fw "$BIN_PATH"
    --fw-config "$DIR/output/morello_fw_config.dtb"
    --hw-config "$DIR/output/morello-fvp.dtb"
    --tb-fw-config "$DIR/output/morello_tb_fw_config.dtb"
    --nt-fw-config "$DIR/output/morello_nt_fw_config.dtb"
    --trusted-key-cert "$DIR/output/trusted_key.crt"
    --soc-fw-key-cert "$DIR/output/bl31_key.crt"
    --nt-fw-key-cert "$DIR/output/bl33_key.crt"
    --soc-fw-cert "$DIR/output/bl31.crt"
    --nt-fw-cert "$DIR/output/bl33.crt"
    --tb-fw-cert "$DIR/output/bl2.crt"
)

cert_flags=(
    -n --tfw-nvctr 0 --ntfw-nvctr 0
    --rot-key "$DIR/trusted-firmware-a/plat/arm/board/common/rotpk/arm_rotprivk_rsa.pem"
)

mkdir -p $OUT_PATH
"$DIR/trusted-firmware-a/tools/cert_create/cert_create" "${common_flags[@]}" "${cert_flags[@]}"
"$DIR/trusted-firmware-a/tools/fiptool/fiptool" create "${common_flags[@]}" "$OUT_PATH/fip.bin"

