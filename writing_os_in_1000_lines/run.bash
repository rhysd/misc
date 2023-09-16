#!/bin/bash

set -xuo pipefail

if [ ! -f opensbi-riscv32-generic-fw_dynamic.bin ]; then
    curl -LO https://github.com/qemu/qemu/raw/v8.0.4/pc-bios/opensbi-riscv32-generic-fw_dynamic.bin
fi

set -x

# -nographic        : No window
# -serial mon:stdio : Connect stdio to serial port
# --no-reboot       : Do not reboot on panic
qemu-system-riscv32 -machine virt -bios default -nographic -serial mon:stdio --no-reboot -kernel kernel.elf
