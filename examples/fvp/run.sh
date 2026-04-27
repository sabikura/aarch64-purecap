#!/usr/bin/env bash

set -euo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"

if [ $# -lt 1 ]; then
    echo "usage: $0 <path-to-fip>" >&2
    exit 1
fi

FIP_BIN="$1"

: "${CHERI_HOME:=$HOME/cheri}"
FVP="$CHERI_HOME/output/morello-sdk/FVP_Morello/models/Linux64_GCC-6.4/FVP_Morello"

"$FVP" \
  --data "Morello_Top.css.scp.armcortexm7ct=$DIR/firmware/scp_romfw.bin@0x0" \
  --data "Morello_Top.css.mcp.armcortexm7ct=$DIR/firmware/mcp_romfw.bin@0x0" \
  -C "Morello_Top.soc.scp_qspi_loader.fname=$DIR/firmware/scp_fw.bin" \
  -C "Morello_Top.soc.mcp_qspi_loader.fname=$DIR/firmware/mcp_fw.bin" \
  -C "css.scp.armcortexm7ct.INITVTOR=0x0" \
  -C "css.mcp.armcortexm7ct.INITVTOR=0x0" \
  -C "css.trustedBootROMloader.fname=/home/irina/personal/fvp-firmware/bl1.bin" \
  -C "board.ap_qspi_loader.fname=$FIP_BIN" \
  -C "css.pl011_uart_ap.unbuffered_output=1" \
  -C "num_clusters=1" \
  -C "num_cores=1" \
  -C "displayController=0" \
  -C "disable_visualisation=true" \
  -C "board.terminal_uart0_board.start_telnet=0" \
  -C "board.terminal_uart1_board.start_telnet=0" \
  -C "css.mcp.terminal_uart0.start_telnet=0" \
  -C "css.mcp.terminal_uart1.start_telnet=0" \
  -C "css.scp.terminal_uart_aon.start_telnet=0" \
  -C "css.terminal_sec_uart_ap.start_telnet=0" \
  -C "css.terminal_uart1_ap.start_telnet=0" \
  -C "css.terminal_uart_ap.start_telnet=0"

