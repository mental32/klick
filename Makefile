QEMU = qemu
GRUB = grub
NASM = nasm
LD   = ld

arch   ?= x86_64
target ?= target-$(arch)

iso     := build/klick-$(arch).iso
kernel  := build/klick-$(arch).bin
rust_os := target/$(target)/debug/libklick.a

grub_cfg := common/grub.cfg

linker_script         := src/arch/$(arch)/linker.ld
assembly_source_files := $(wildcard src/arch/$(arch)/asm/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/asm/%.asm, build/arch/$(arch)/asm/%.o, $(assembly_source_files))

.PHONY: all clean run iso kernel

all: $(kernel)

clean:
	@rm -r build
	@cargo clean

run: $(iso)
	@$(QEMU)-system-x86_64 $(iso) -m 4G -d int -machine smm=off

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@$(GRUB)-mkrescue -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): kernel $(rust_os) $(assembly_object_files)
	@$(LD) -n --gc-sections -T $(linker_script) -o $(kernel) \
		$(assembly_object_files) $(rust_os)

kernel:
	@cp common/$(target).json $(target).json
	@RUST_TARGET_PATH="$(shell pwd)" xargo build --target $(target)
	@rm $(target).json

# compile assembly files
build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@$(NASM) -felf64 $< -o $@
