# Makefile for linking the EL2 entry and the Rust EL1 runtime to
# build an BL33 image for the TF-A

MKFILE_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

# Toolchain
CC      := $(MKFILE_DIR)tools/clang.sh
AS      := $(MKFILE_DIR)tools/clang.sh
LD      := $(MKFILE_DIR)tools/ld.sh
OBJCOPY := $(MKFILE_DIR)tools/objcopy.sh
OBJDUMP := $(MKFILE_DIR)tools/objdump.sh

# Compiler flags
TARGET       := aarch64-none-elf
ARCH_COMMON  := -target $(TARGET) -march=morello -mabi=purecap

# Objects
TARGET_DIR    := $(MKFILE_DIR)target/
EL2_ENTRY_OBJ := $(OUTPUT_DIR)entry.o
EL2_ENTRY_ASM := entry.S

$(EL2_ENTRY_OBJ): $(EL2_ENTRY_ASM)
	$(AS) $(ARCH_COMMON) -c $< -o $@

# $(EL1_RS_OBJ):

.PHONY: dump
dump: $(IMAGE)
	$(OBJDUMP) -d $<
