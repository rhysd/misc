CC := clang
CFLAGS := -std=c11 -O2 -g3 -Wall -Wextra --target=riscv32 -ffreestanding -nostdlib
OBJCOPY := llvm-objcopy
FS := $(wildcard disk/*)

all: kernel.elf shell.bin.o disk.tar opensbi-riscv32-generic-fw_dynamic.bin

kernel.elf: kernel.ld kernel.c kernel.h common.c common.h shell.bin.o
	$(CC) $(CFLAGS) -Wl,-Tkernel.ld -Wl,-Map=kernel.map -o kernel.elf \
		kernel.c common.c shell.bin.o

shell.elf: user.ld shell.c user.c user.h common.c common.h
	$(CC) $(CFLAGS) -Wl,-Tuser.ld -Wl,-Map=shell.map -o shell.elf \
		shell.c user.c common.c
shell.bin: shell.elf
	$(OBJCOPY) --set-section-flags .bss=alloc,contents -O binary shell.elf shell.bin
shell.bin.o: shell.bin
	$(OBJCOPY) -Ibinary -Oelf32-littleriscv shell.bin shell.bin.o

disk.tar: $(FS)
	(cd ./disk && tar cf ../disk.tar --format=ustar ./*.txt)

opensbi-riscv32-generic-fw_dynamic.bin:
	curl -LO https://github.com/qemu/qemu/raw/v8.0.4/pc-bios/opensbi-riscv32-generic-fw_dynamic.bin

clean:
	rm *.elf *.map *.bin *.o disk.tar

.PHONY: clean all
