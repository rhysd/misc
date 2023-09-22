#!/bin/bash

set -xeuo pipefail

# -nographic        : No window
# -serial mon:stdio : Connect stdio to serial port
# --no-reboot       : Do not reboot on panic
# -drive            : Define disk drive
# -device           : Add device
qemu-system-riscv32 -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
    -d unimp,guest_errors,int,cpu_reset -D qemu.log \
    -drive id=drive0,file=disk.tar,format=raw \
    -device virtio-blk-device,drive=drive0,bus=virtio-mmio-bus.0 \
    -kernel kernel.elf
