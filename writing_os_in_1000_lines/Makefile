kernel.elf: kernel.c kernel.h common.c common.h
	clang \
		-std=c11 -O2 -g3 -Wall -Wextra --target=riscv32 -ffreestanding -nostdlib \
		-Wl,-Tkernel.ld -Wl,-Map=kernel.map -o kernel.elf \
		kernel.c common.c
