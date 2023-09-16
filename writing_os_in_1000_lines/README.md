Writing OS in 1000 lines
========================

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
└── run.sh    - Build script
```
