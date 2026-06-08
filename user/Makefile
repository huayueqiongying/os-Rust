TARGET := riscv64gc-unknown-none-elf
MODE := release
APP_DIR := src/bin
TARGET_DIR := target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

OBJDUMP := rust-objdump --arch-name=riscv64

elf: $(APPS)
	@cargo build --release

build: elf

clean:
	@cargo clean

.PHONY: elf build clean