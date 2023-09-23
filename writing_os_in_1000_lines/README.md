Writing OS in 1000 lines
========================

This is a very small toy OS written in about 1000 lines following the below guide.

https://operating-system-in-1000-lines.vercel.app/ja/welcome

```console
$ tokei *.c *.h *.ld
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language            Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 C                       6         1112          861          114          137
 C Header                3           98           75            8           15
 LD Script               2           78           48           15           15
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                  11         1288          984          137          167
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

### Files

- `common.c`, `common.h` : Common libraries shared by both kernel and user (e.g. `printf`, `memset`)
- Kernel land
  - `kernel.h` : Constants and declarations shared within kernel
  - `kernel.c` : Entry point, allocation, paging, system calls, exception handler
  - `fs.c` : Device (virtio-blk) driver and filesystem
  - `proc.c` : Multiple processes management with context switch
  - `kernel.ld` : Linker script for memory layout of kernel land
- User land
  - `stdlib.c`, `stdlib.h` : Standard library
  - `shell.c` : Command line shell (user program)
  - `user.ld` : Linker script for memory layout of user land
- `disk/*.txt` : Filesystem content
- `Makefile` : Build script
- `run.bash` : Script to run kernel with QEMU
- `env.sh` : Script to prepare development environment (for macOS)

### How to build

```sh
source env.sh  # For macOS
make
```

### How to run QEMU

```sh
./run.bash
```

### References

- RISC-V specification: https://github.com/riscv/riscv-isa-manual/releases/download/Priv-v1.12/riscv-privileged-20211203.pdf
- QEMU `virt` machine: https://www.qemu.org/docs/master/system/riscv/virt.html
- Open SBI specification releases: https://github.com/riscv-non-isa/riscv-sbi-doc/releases
- VIRTIO specification: https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html
