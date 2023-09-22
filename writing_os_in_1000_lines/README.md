Writing OS in 1000 lines
========================

This is a very small toy OS written in about 1000 lines following the below guide.

https://operating-system-in-1000-lines.vercel.app/ja/welcome

```console
$ tokei *.c *.h                                                                                                                                                                                                                                                                              [Husky.local][09/22 23:09]
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 C                       4          948          715          108          125
 C Header                3          247          206           15           26
===============================================================================
 Total                   7         1195          921          123          151
===============================================================================
```

### Repository structure

```
├── disk/     - Filesystem content
├── common.c  - Common libraries shared by both kernel and (e.g. `printf`, `memset`)
├── common.h  - Common libraries shared by both kernel and (structs and constants)
├── kernel.c  - Kernel: Process management, system call, device driver, filesystem
├── kernel.h  - Kernel: structs and constants
├── kernel.ld - Kernel: Linker script (definition of memory layout)
├── shell.c   - Command line shell
├── user.c    - Libraries for user land: Functions for using system calls, ...
├── user.h    - Libraries for user land: Structs and constants
├── user.ld   - User land: Linker script (definition of memory layout)
├── Makefile  - Build script
└── run.sh    - Script to run kernel with QEMU
```

### How to build

```sh
source env.sh  # For macOS
make
```

### How to run QEMU

```sh
./run.bash
```
