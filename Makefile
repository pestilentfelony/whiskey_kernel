RUSTC   = rustc
SYSROOT = $(shell rustc --print sysroot)

CROSS_PREFIX ?= $(shell if command -v riscv64-elf-as >/dev/null 2>&1; then echo riscv64-elf-; elif command -v riscv64-linux-gnu-as >/dev/null 2>&1; then echo riscv64-linux-gnu-; else echo riscv64-elf-; fi)
AS = $(CROSS_PREFIX)as
LD = $(CROSS_PREFIX)ld

TARGET = riscv64gc-unknown-none-elf
RUSTFLAGS = --target $(TARGET) --sysroot $(SYSROOT) -O -C panic=abort

BIN = build/kernel.elf

all: $(BIN)

$(BIN): src/entry.s src/main.rs linker.ld
	@mkdir -p build
	# Assemble the bootstrapper
	$(AS) src/entry.s -o build/entry.o
	# Compile the Rust code into an object file
	$(RUSTC) $(RUSTFLAGS) --emit=obj src/main.rs -o build/main.o
	# Link them manually using the script
	$(LD) -T linker.ld build/entry.o build/main.o -o $(BIN)

run: $(BIN)
	qemu-system-riscv64 -machine virt -bios none -kernel build/kernel.elf -nographic

clean:
	rm -rf build
