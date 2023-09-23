#include "kernel.h"

struct sbiret {
    long error;
    long value;
};

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

extern char __bss[], __bss_end[], __stack_top[];

// Start and end address of the raw binary shell.bin.o
extern char _binary_shell_bin_start[], _binary_shell_bin_size[];

// Chapter 3. Binary Encoding
//   fid = SBI function ID, eid = SBI extension ID
struct sbiret sbi_call(long arg0, long arg1, long arg2, long arg3, long arg4,
                       long arg5, long fid, long eid) {
    register long a0 __asm__("a0") = arg0;
    register long a1 __asm__("a1") = arg1;
    register long a2 __asm__("a2") = arg2;
    register long a3 __asm__("a3") = arg3;
    register long a4 __asm__("a4") = arg4;
    register long a5 __asm__("a5") = arg5;
    register long a6 __asm__("a6") = fid;
    register long a7 __asm__("a7") = eid;

    __asm__ __volatile__("ecall"
                         : "=r"(a0), "=r"(a1)
                         : "r"(a0), "r"(a1), "r"(a2), "r"(a3), "r"(a4), "r"(a5),
                           "r"(a6), "r"(a7)
                         : "memory");
    struct sbiret ret;
    ret.error = a0;
    ret.value = a1;
    return ret;
}

// 5.2. Extension: Console Putchar (EID #0x01)
void putchar(char ch) {
    sbi_call(ch, 0, 0, 0, 0, 0, 0, 1 /* Console Putchar */);
}

// 5.3. Extension: Console Getchar (EID #0x02)
long getchar() {
    struct sbiret ret = sbi_call(0, 0, 0, 0, 0, 0, 0, 2 /* Console Getchar */);
    return ret.error;
}

// Exception handler.
// - Save and restore registers using stack. The fp registers are not needed to be saved because they are not used by kernel
// - The sscratch register is used to save/restore stack pointer
__attribute__((naked))
__attribute__((aligned(4))) // because stvec contains the execution mode in the lower 2 bits
void
exception_handler(void) {
    __asm__ __volatile__(
        // Get the bottom address of the current process's kernel stack from sscratch register which was saved to
        // sscratch register in yield() function. Instead, the current stack pointer (where the exception happened)
        // is saved to sscratch register.
        //   tmp = sp; sp = sscratch; sscratch = tmp;
        "csrrw sp, sscratch, sp\n"

        // Save registers before calling trap handler
        "addi sp, sp, -4 * 31\n"
        "sw ra,  4 * 0(sp)\n"
        "sw gp,  4 * 1(sp)\n"
        "sw tp,  4 * 2(sp)\n"
        "sw t0,  4 * 3(sp)\n"
        "sw t1,  4 * 4(sp)\n"
        "sw t2,  4 * 5(sp)\n"
        "sw t3,  4 * 6(sp)\n"
        "sw t4,  4 * 7(sp)\n"
        "sw t5,  4 * 8(sp)\n"
        "sw t6,  4 * 9(sp)\n"
        "sw a0,  4 * 10(sp)\n"
        "sw a1,  4 * 11(sp)\n"
        "sw a2,  4 * 12(sp)\n"
        "sw a3,  4 * 13(sp)\n"
        "sw a4,  4 * 14(sp)\n"
        "sw a5,  4 * 15(sp)\n"
        "sw a6,  4 * 16(sp)\n"
        "sw a7,  4 * 17(sp)\n"
        "sw s0,  4 * 18(sp)\n"
        "sw s1,  4 * 19(sp)\n"
        "sw s2,  4 * 20(sp)\n"
        "sw s3,  4 * 21(sp)\n"
        "sw s4,  4 * 22(sp)\n"
        "sw s5,  4 * 23(sp)\n"
        "sw s6,  4 * 24(sp)\n"
        "sw s7,  4 * 25(sp)\n"
        "sw s8,  4 * 26(sp)\n"
        "sw s9,  4 * 27(sp)\n"
        "sw s10, 4 * 28(sp)\n"
        "sw s11, 4 * 29(sp)\n"

        // Push the sp (where the exception happened) to stack
        "csrr a0, sscratch\n"
        "sw a0, 4 * 30(sp)\n"

        // Reset the kernel stack to sscratch register again
        "addi a0, sp, 4 * 31\n"
        "csrw sscratch, a0\n"

        "mv a0, sp\n"
        "call handle_trap\n"

        // Restore registers after calling trap handler
        "lw ra,  4 * 0(sp)\n"
        "lw gp,  4 * 1(sp)\n"
        "lw tp,  4 * 2(sp)\n"
        "lw t0,  4 * 3(sp)\n"
        "lw t1,  4 * 4(sp)\n"
        "lw t2,  4 * 5(sp)\n"
        "lw t3,  4 * 6(sp)\n"
        "lw t4,  4 * 7(sp)\n"
        "lw t5,  4 * 8(sp)\n"
        "lw t6,  4 * 9(sp)\n"
        "lw a0,  4 * 10(sp)\n"
        "lw a1,  4 * 11(sp)\n"
        "lw a2,  4 * 12(sp)\n"
        "lw a3,  4 * 13(sp)\n"
        "lw a4,  4 * 14(sp)\n"
        "lw a5,  4 * 15(sp)\n"
        "lw a6,  4 * 16(sp)\n"
        "lw a7,  4 * 17(sp)\n"
        "lw s0,  4 * 18(sp)\n"
        "lw s1,  4 * 19(sp)\n"
        "lw s2,  4 * 20(sp)\n"
        "lw s3,  4 * 21(sp)\n"
        "lw s4,  4 * 22(sp)\n"
        "lw s5,  4 * 23(sp)\n"
        "lw s6,  4 * 24(sp)\n"
        "lw s7,  4 * 25(sp)\n"
        "lw s8,  4 * 26(sp)\n"
        "lw s9,  4 * 27(sp)\n"
        "lw s10, 4 * 28(sp)\n"
        "lw s11, 4 * 29(sp)\n"
        "lw sp,  4 * 30(sp)\n"
        "sret\n");
}

paddr_t alloc_pages(uint32_t const n) {
    static paddr_t next_paddr = (paddr_t)__free_ram;
    paddr_t paddr = next_paddr;
    next_paddr += n * PAGE_SIZE;

    if (next_paddr > (paddr_t)__free_ram_end) {
        PANIC("out of memory");
    }

    memset((void *)paddr, 0, n * PAGE_SIZE);
    return paddr;
}

// Register the mapping between the virtual address and the physical address in the page table
void map_page(uint32_t *table1, uint32_t vaddr, paddr_t paddr, uint32_t flags) {
    // Sv32 is a two-phase lookup table. It means a tree structure where depth is 3.
    // Depth-1 is VPN[1], depth-2 is VPN[0], depth-3 is the page offset in the 4KiB page.
    //
    // Example of Sv32 page mapping
    // ┌──────────────────────────────────┐
    // │             8001123              │ Virtual address
    // └──────────────────────────────────┘
    // ┌──────────┬──────────┬────────────┐
    // │0000100000│0000000001│000100100011│ Page table entry
    // └──────────┴──────────┴────────────┘
    //    VPN[1]     VPN[0]     page offset
    //   (10 bits)  (10 bits)  (12 bits)
    //     =32        =1         =0x123

    if (!is_aligned(vaddr, PAGE_SIZE)) {
        PANIC("unaligned vaddr %x", vaddr);
    }

    if (!is_aligned(paddr, PAGE_SIZE)) {
        PANIC("unaligned paddr %x", paddr);
    }

    uint32_t const vpn1 = (vaddr >> (10 + 12)) & 0x3ff; // Extract the first page table index
    if ((table1[vpn1] & PAGE_V) == 0) {
        // The second page table does not exist. Create it.
        uint32_t const pt_paddr = alloc_pages(1);
        table1[vpn1] = ((pt_paddr / PAGE_SIZE) << 10) | PAGE_V;
    }

    // Insert the new page entry to the second page table
    uint32_t const vpn0 = (vaddr >> 12) & 0x3ff; // Extract the second page table index
    uint32_t *table0 = (uint32_t *)((table1[vpn1] >> 10) * PAGE_SIZE);
    table0[vpn0] = ((paddr / PAGE_SIZE) << 10) | flags | PAGE_V;
}

void handle_syscall(struct trap_frame *f) {
    switch (f->a3) {
    case SYSCALL_PUTCHAR:
        putchar(f->a0);
        break;
    case SYSCALL_GETCHAR:
        for (;;) {
            long const ch = getchar();
            if (ch >= 0) {
                // When some character is returned, write it to a0 and return from syscall
                f->a0 = ch;
                break;
            }
            // When return value is -1, it means that no character was input. Try getchar() again.
            // However, busy loop stops entire kernel and prevents all processes execution.
            // To avoid it, yield the execution to some runnable process then try getchar() later.
            yield();
        }
        break;
    case SYSCALL_EXIT:
        exit();
        break;
    case SYSCALL_READFILE:
    case SYSCALL_WRITEFILE: {
        char const *filename = (char const *)f->a0;
        char *buf = (char *)f->a1;
        int len = f->a2;
        struct file *file = fs_lookup(filename);
        if (!file) {
            printf("file not found: %s\n", filename);
            f->a0 = -1;
            break;
        }
        if (len > (int)sizeof(file->data)) {
            len = file->size;
        }
        if (f->a3 == SYSCALL_WRITEFILE) {
            memcpy(file->data, buf, len);
            file->size = len;
            fs_flush();
        } else {
            memcpy(buf, file->data, len);
        }
        f->a0 = len; // Return write/read length
        break;
    }
    default:
        PANIC("unexpected syscall a3=%x\n", f->a3);
    }
}

void handle_trap(struct trap_frame *f) {
    uint32_t scause, stval, user_pc;
    READ_CSR(scause, scause);
    READ_CSR(stval, stval);
    READ_CSR(user_pc, sepc);

    switch (scause) {
    case SCAUSE_ECALL:
        handle_syscall(f);
        // Increment PC to the next to ecall instruction
        user_pc += 4;
        break;
    default:
        PANIC("unexpected trap scause=%x, stval=%x, sepc=%x\n", scause, stval, user_pc);
        break;
    }

    WRITE_CSR(sepc, user_pc);
}

void kernel_main(void) {
    // There is no guarantee that bootloader initializes bss with zeros
    memset(__bss, 0, (size_t)__bss_end - (size_t)__bss);

    // Set exception handler
    WRITE_CSR(stvec, (uint32_t)exception_handler);

    virtio_blk_init(); // Initialize virtio-blk device and read disk contents for filesystem
    fs_init();
    proc_init();

    // Run shell.bin.o
    create_process(_binary_shell_bin_start, (size_t)_binary_shell_bin_size);

    yield(); // Try to run the first runnable process
    PANIC("switched to idle process");
}

// Kernel entrypoint
__attribute__((section(".text.boot")))
__attribute__((naked)) // Do not generate prologue and epilogue
void
boot(void) {
    __asm__ __volatile__(
        "mv sp, %[stack_top]\n"
        "j kernel_main\n"
        :
        : [stack_top] "r"(__stack_top));
}
