#pragma once

#include "common.h"

#define PAGE_SIZE 4096 // Page size is 4KiB

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
// 4.1.1.2 Memory Privilege in sstatus Register
// SUM (permit Supervisor User Memory access) bit. Until this bit is set to sstatus, kernel cannot
// access to user's pages.
#define SSTATUS_SUM (1 << 18)
// Environment call from U-mode. See Table 4.2 in 4.1.8 Supervisor Cause Register (scause)
#define SCAUSE_ECALL 8

#define PANIC(fmt, ...)                                                       \
    do {                                                                      \
        printf("PANIC: %s:%d: " fmt "\n", __FILE__, __LINE__, ##__VA_ARGS__); \
        while (1) {                                                           \
        }                                                                     \
    } while (0)

#define VIRTIO_BLK_PADDR 0x10001000

struct file {
    bool in_use;
    char name[100];
    char data[1024];
    size_t size;
};

extern char __free_ram[], __free_ram_end[], __kernel_base[];

paddr_t alloc_pages(uint32_t const n);
void map_page(uint32_t *table1, uint32_t vaddr, paddr_t paddr, uint32_t flags);
void virtio_blk_init(void);
void fs_init(void);
void fs_flush(void);
struct file *fs_lookup(char const *filename);
struct process *create_process(void const *image, size_t const image_size);
void yield(void);
void proc_init();
__attribute__((noreturn)) void exit();
