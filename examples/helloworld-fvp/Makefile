# Makefile for linking the EL2 entry and the Rust EL1 runtime to
# build an BL33 image for the TF-A

MKFILE_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

# Toolchain
CC      := $(MKFILE_DIR)tools/clang.sh
LD      := $(MKFILE_DIR)tools/ld.sh
OBJCOPY := $(MKFILE_DIR)tools/objcopy.sh
OBJDUMP := $(MKFILE_DIR)tools/objdump.sh
CARGO   := $(MKFILE_DIR)tools/cargo.sh

# Flags
TARGET        := aarch64-none-elf
ARCH_COMMON   := -target $(TARGET) -march=morello
OBJDUMP_FLAGS := -d --triple=aarch64-none-elf --mattr=+morello
EL2_LD_FLAGS  += -T $(MKFILE_DIR)el2.ld

# Source files
SRCS	  := $(wildcard src/*.rs)
EL2_ENTRY_ASM := $(MKFILE_DIR)src/el2/entry.S

# Binaries
IMAGE         := $(MKFILE_DIR)build/bin/helloworld.bin
KERNEL_BIN    := $(MKFILE_DIR)build/bin/kernel.bin

EL2_ENTRY_OBJ := $(MKFILE_DIR)build/elf/entry.o
EL2_ENTRY_ELF := $(MKFILE_DIR)build/elf/entry.elf

$(EL2_ENTRY_ELF): $(EL2_ENTRY_ASM) | $(MKFILE_DIR)build/elf 
	$(CC) $(ARCH_COMMON) -c $< -o $(EL2_ENTRY_OBJ)
	$(LD) $(EL2_LD_FLAGS) -o $@ $(EL2_ENTRY_OBJ)
	@rm $(EL2_ENTRY_OBJ)

$(MKFILE_DIR)build/elf $(MKFILE_DIR)build/bin:
	mkdir -p $@

EL2_ENTRY_BIN := $(MKFILE_DIR)build/bin/el2.bin

$(EL2_ENTRY_BIN): $(EL2_ENTRY_ELF) | $(MKFILE_DIR)build/bin
	$(OBJCOPY) -O binary $(EL2_ENTRY_ELF) $@
	@truncate -s 2048 $@

KERNEL_ELF    := $(MKFILE_DIR)build/elf/kernel.elf

$(KERNEL_ELF): | $(MKFILE_DIR)build/elf
	@$(MKFILE_DIR)tools/cargo.sh b --release
	@cp $(MKFILE_DIR)target/aarch64-unknown-none-purecap/release/helloworld-fvp $(MKFILE_DIR)build/elf/kernel.elf

$(KERNEL_BIN): $(KERNEL_ELF) | $(MKFILE_DIR)build/bin
	@$(OBJCOPY) -O binary $< $@

$(IMAGE): $(KERNEL_BIN) $(EL2_ENTRY_BIN) 
	@cat $(EL2_ENTRY_BIN) > $(IMAGE)
	@cat $(KERNEL_BIN) >> $(IMAGE)

.PHONY: setup
setup:
	docker build -f tools/docker/Dockerfile -t morello-tfa tools/docker/

.PHONY: tfa
tfa: $(IMAGE)
	docker run -v ./build:/root/workspace/build --rm -it morello-tfa sh -c \
		"cp /root/workspace/build/bin/helloworld.bin /root/workspace/output/fvp/intermediates/helloworld.bin && /root/workspace/build-scripts/build-firmware-image.sh -f none -p fvp all && \
		 cp -r /root/workspace/output/fvp/firmware /root/workspace/bsp/rom-binaries /root/workspace/build/ && chown -R $(shell id -u):$(shell id -g) build/"

.PHONY: elf
elf: $(KERNEL_ELF) $(EL2_ENTRY_ELF)

.PHONY: bin
bin: $(IMAGE)

KERNEL_DUMP := $(MKFILE_DIR)build/elf/kernel.dump

.PHONY: dump
dump: $(KERNEL_ELF)
	$(OBJDUMP) $(OBJDUMP_FLAGS) $< > $(KERNEL_DUMP)

FVP 	  := $(MKFILE_DIR)tools/morello-fvp.sh
AP_ROMFW  := $(MKFILE_DIR)build/rom-binaries/bl1.bin
FIP_BIN   := $(MKFILE_DIR)build/firmware/fip.bin
SCP_ROMFW := $(MKFILE_DIR)build/rom-binaries/scp_romfw.bin
MCP_ROMFW := $(MKFILE_DIR)build/rom-binaries/mcp_romfw.bin
SCP_RAMFW := $(MKFILE_DIR)build/firmware/scp_fw.bin
MCP_RAMFW := $(MKFILE_DIR)build/firmware/mcp_fw.bin

# TODO: Add dependencies here
.PHONY: run
run:
	$(FVP) \
	  --data "Morello_Top.css.scp.armcortexm7ct=$(SCP_ROMFW)@0x0" \
	  --data "Morello_Top.css.mcp.armcortexm7ct=$(MCP_ROMFW)@0x0" \
	  -C "css.cluster0.cpu0.semihosting-enable=1" \
	  -C "Morello_Top.soc.scp_qspi_loader.fname=$(SCP_RAMFW)" \
	  -C "Morello_Top.soc.mcp_qspi_loader.fname=$(MCP_RAMFW)" \
	  -C "css.scp.armcortexm7ct.INITVTOR=0x0" \
	  -C "css.mcp.armcortexm7ct.INITVTOR=0x0" \
	  -C "css.trustedBootROMloader.fname=$(AP_ROMFW)" \
	  -C "board.ap_qspi_loader.fname=$(FIP_BIN)" \
	  -C "css.pl011_uart_ap.unbuffered_output=1" \
	  -C "displayController=1" \
	  -C "num_clusters=1" \
	  -C "num_cores=1" \
	  -C "css.scp.pl011_uart_scp.out_file=scp.log" \
	  -C "css.mcp.pl011_uart0_mcp.out_file=mcp.log" \
	  -C "displayController=0" \
	  -C "board.terminal_uart0_board.start_telnet=0" \
      -C "board.terminal_uart1_board.start_telnet=0" \
      -C "css.mcp.terminal_uart0.start_telnet=0" \
      -C "css.mcp.terminal_uart1.start_telnet=0" \
      -C "css.scp.terminal_uart_aon.start_telnet=0" --stat
  # -C "css.terminal_sec_uart_ap.start_telnet=0" \
  # -C "css.terminal_uart1_ap.start_telnet=0" \
  # -C "css.terminal_uart_ap.start_telnet=0"
  # -C css.pl011_uart_ap.out_file=ap.log \

.PHONY: clean
clean:
	rm -rf $(MKFILE_DIR)build/bin $(MKFILE_DIR)build/elf
