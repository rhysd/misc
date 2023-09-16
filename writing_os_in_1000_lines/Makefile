kernel.elf: kernel.c
	clang \
		-std=c11 -O2 -g3 -Wall -Wextra --target=riscv32 -ffreestanding -nostdlib \
		-Wl,-Tkernel.ld -Wl,-Map=kernel.map -o kernel.elf \
		kernel.c
