MKFILE_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

# Toolchain
CC      := $(MKFILE_DIR)tools/clang.sh
AS      := $(MKFILE_DIR)tools/clang.sh
LD      := $(MKFILE_DIR)tools/ld.sh
OBJCOPY := $(MKFILE_DIR)tools/objcopy.sh
OBJDUMP := $(MKFILE_DIR)tools/objdump.sh

# Objects
BUILD_DIR     := $(MKFILE_DIR)build/
EL2_ENTRY_OBJ := $(BUILD_DIR)entry.o
EL2_ENTRY_ELF := $(BUILD_DIR)entry.elf
EL2_ENTRY_ASM := entry.S
LINKER_SCRIPT := link.ld

# Compiler flags
TARGET        := aarch64-none-elf
ARCH_COMMON   := -target $(TARGET) -march=morello
LD_FLAGS      += -T $(LINKER_SCRIPT)
OBJDUMP_FLAGS := -d --triple=aarch64-none-elf --mattr=+morello

$(EL2_ENTRY_ELF): $(EL2_ENTRY_OBJ)
	$(LD) $(LD_FLAGS) -o $@ $<

$(EL2_ENTRY_OBJ): $(EL2_ENTRY_ASM)
	@mkdir -p $(dir $@)
	$(AS) $(ARCH_COMMON) -c $< -o $@

.PHONY: build
build: $(EL2_ENTRY_ELF)	

.PHONY: dump
dump: $(EL2_ENTRY_ELF)
	$(OBJDUMP) $(OBJDUMP_FLAGS) $< > $(BUILD_DIR)entry.dump

.PHONY: clean
clean:
	@rm -rf $(BUILD_DIR)
