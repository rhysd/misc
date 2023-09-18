#pragma once

#include "common.h"

#define PAGE_SIZE 4096              // Page size is 4KiB
#define PROCS_MAX 8                 // Maximum number of processes
#define PROC_STATE_UNUSED 0         // Process is unused
#define PROC_STATE_RUNNABLE 1       // Process is runnable
#define PROC_STATE_EXITED 2         // Process exited
#define PROC_KERNEL_STACK_SIZE 8192 // Stack size of process in bytes

#define SATP_SV32 (1u << 31) // The satp register bit to indicate to enable Sv32 mode paging
#define PAGE_V (1 << 0)      // The page is enabled
#define PAGE_R (1 << 1)      // The page is readable
#define PAGE_W (1 << 2)      // The page is writable
#define PAGE_X (1 << 3)      // The page is executable
#define PAGE_U (1 << 4)      // The page is accessible from user land

// Base address of user land. This must match to the address in user.ld. This magic number is necessary
// since we use raw binary format. If we use general executable format like ELF, we can know this address
// from the executable's header.
#define USER_BASE_ADDR 0x1000000
// 4.1.1 Supervisor Status Register (sstatus)
#define SSTATUS_SPIE (1 << 5)
// Environment call from U-mode. See Table 4.2 in 4.1.8 Supervisor Cause Register (scause)
#define SCAUSE_ECALL 8

struct sbiret {
    long error;
    long value;
};

struct process {
    int pid;                               // Process ID
    int state;                             // ...
    vaddr_t sp;                            // Stack pointer on context switch
    uint32_t *page_table;                  // Page table for virtual address mapping
    uint8_t stack[PROC_KERNEL_STACK_SIZE]; // Kernel stack
};

#define PANIC(fmt, ...)                                                       \
    do {                                                                      \
        printf("PANIC: %s:%d: " fmt "\n", __FILE__, __LINE__, ##__VA_ARGS__); \
        while (1) {                                                           \
        }                                                                     \
    } while (0)

struct trap_frame {
    uint32_t ra;
    uint32_t gp;
    uint32_t tp;
    uint32_t t0;
    uint32_t t1;
    uint32_t t2;
    uint32_t t3;
    uint32_t t4;
    uint32_t t5;
    uint32_t t6;
    uint32_t a0;
    uint32_t a1;
    uint32_t a2;
    uint32_t a3;
    uint32_t a4;
    uint32_t a5;
    uint32_t a6;
    uint32_t a7;
    uint32_t s0;
    uint32_t s1;
    uint32_t s2;
    uint32_t s3;
    uint32_t s4;
    uint32_t s5;
    uint32_t s6;
    uint32_t s7;
    uint32_t s8;
    uint32_t s9;
    uint32_t s10;
    uint32_t s11;
    uint32_t sp;
} __attribute__((packed));

#define READ_CSR(var, reg)                    \
    do {                                      \
        unsigned long __tmp;                  \
        __asm__ __volatile__("csrr %0, " #reg \
                             : "=r"(__tmp));  \
        var = __tmp;                          \
    } while (0)

#define WRITE_CSR(reg, value)                                   \
    do {                                                        \
        uint32_t __tmp = (value);                               \
        __asm__ __volatile__("csrw " #reg ", %0" ::"r"(__tmp)); \
    } while (0)
