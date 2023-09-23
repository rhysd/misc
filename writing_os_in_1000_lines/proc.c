#include "kernel.h"

#define PROCS_MAX 8                 // Maximum number of processes
#define PROC_STATE_UNUSED 0         // Process is unused
#define PROC_STATE_RUNNABLE 1       // Process is runnable
#define PROC_STATE_EXITED 2         // Process exited
#define PROC_KERNEL_STACK_SIZE 8192 // Stack size of process in bytes

struct process {
    int pid;                               // Process ID
    int state;                             // Current state of process (unused/runnable/exited)
    vaddr_t sp;                            // Stack pointer on context switch
    uint32_t *page_table;                  // Page table for virtual address mapping
    uint8_t stack[PROC_KERNEL_STACK_SIZE]; // Kernel stack
};

static struct process *current_proc;    // The process currently running
static struct process *idle_proc;       // The idle process (PID=-1) which runs when no process is runnable
static struct process procs[PROCS_MAX]; // Note: All elements are zero-initialized so the state is PROC_STATE_UNUSED

__attribute__((naked)) static void switch_context(uint32_t *prev_sp, uint32_t *next_sp) {
    __asm__ __volatile__(
        // Save ra and saved registers on current process's stack
        "addi sp, sp, -13 * 4\n"
        "sw ra,  0  * 4(sp)\n"
        "sw s0,  1  * 4(sp)\n"
        "sw s1,  2  * 4(sp)\n"
        "sw s2,  3  * 4(sp)\n"
        "sw s3,  4  * 4(sp)\n"
        "sw s4,  5  * 4(sp)\n"
        "sw s5,  6  * 4(sp)\n"
        "sw s6,  7  * 4(sp)\n"
        "sw s7,  8  * 4(sp)\n"
        "sw s8,  9  * 4(sp)\n"
        "sw s9,  10 * 4(sp)\n"
        "sw s10, 11 * 4(sp)\n"
        "sw s11, 12 * 4(sp)\n"
        "sw sp, (a0)\n" // Save the current stack pointer to `prev_sp` (=a0)

        // Restore ra and saved registers saved on next process's stack
        "lw sp, (a1)\n" // Load the current stack pointer from `next_sp` (=a1)
        "lw ra,  0  * 4(sp)\n"
        "lw s0,  1  * 4(sp)\n"
        "lw s1,  2  * 4(sp)\n"
        "lw s2,  3  * 4(sp)\n"
        "lw s3,  4  * 4(sp)\n"
        "lw s4,  5  * 4(sp)\n"
        "lw s5,  6  * 4(sp)\n"
        "lw s6,  7  * 4(sp)\n"
        "lw s7,  8  * 4(sp)\n"
        "lw s8,  9  * 4(sp)\n"
        "lw s9,  10 * 4(sp)\n"
        "lw s10, 11 * 4(sp)\n"
        "lw s11, 12 * 4(sp)\n"
        "addi sp, sp, 13 * 4\n"
        "ret\n" // Return to next process's ra
    );
}

__attribute__((naked)) static void user_entry(void) {
    __asm__ __volatile__(
        // PC after switching to U-mode
        "csrw sepc, %[sepc]\n"
        // Set SPIE bit. This enables interrupts in U-mode so that trap handler in stvec is called
        // same as exceptions.
        // Note: Our implementation does not use interrupts actually (use polling instead) so this
        // flag is actually unnecessary. But it is a good idea to always enable this bit so that
        // interrupts are not ignored.
        "csrw sstatus, %[sstatus]\n"
        // Switch from S-mode to U-mode
        "sret\n"
        :
        : [sepc] "r"(USER_BASE_ADDR),
          [sstatus] "r"(SSTATUS_SPIE | SSTATUS_SUM));
}

struct process *create_process(void const *image, size_t const image_size) {
    // Search available slot
    struct process *proc = NULL;
    int i;
    for (i = 0; i < PROCS_MAX; i++) {
        if (procs[i].state == PROC_STATE_UNUSED) {
            proc = &procs[i];
            break;
        }
    }

    if (!proc) {
        PANIC("no free process slots");
    }

    uint32_t *sp = (uint32_t *)&proc->stack[sizeof(proc->stack)]; // The bottom address of stack

    // Push first saved register values so that the process can be restored by context switch
    *--sp = 0;                    // s11
    *--sp = 0;                    // s10
    *--sp = 0;                    // s9
    *--sp = 0;                    // s8
    *--sp = 0;                    // s7
    *--sp = 0;                    // s6
    *--sp = 0;                    // s5
    *--sp = 0;                    // s4
    *--sp = 0;                    // s3
    *--sp = 0;                    // s2
    *--sp = 0;                    // s1
    *--sp = 0;                    // s0
    *--sp = (uint32_t)user_entry; // ra

    // Create kernel page mapping (from __kernel_base to __free_ram_end) so that kernel can access
    // to both static area (e.g. .text) and dynamically allocated area by alloc_pages().
    uint32_t *page_table = (uint32_t *)alloc_pages(1);
    for (paddr_t paddr = (paddr_t)__kernel_base; paddr < (paddr_t)__free_ram_end; paddr += PAGE_SIZE) {
        map_page(page_table, paddr, paddr, PAGE_R | PAGE_W | PAGE_X);
    }

    // Map addresses for virtio-blk's Memory-mapped I/O
    map_page(page_table, VIRTIO_BLK_PADDR, VIRTIO_BLK_PADDR, PAGE_R | PAGE_W);

    // Create user page mapping.
    // Note: If memcpy is not used and the image binary is mapped directly, all processes which run
    // the same binary shares the same physical address.
    for (uint32_t offset = 0; offset < image_size; offset += PAGE_SIZE) {
        paddr_t const page = alloc_pages(1);
        memcpy((void *)page, image + offset, PAGE_SIZE);
        map_page(page_table, USER_BASE_ADDR + offset, page, PAGE_U | PAGE_R | PAGE_W | PAGE_X);
    }

    proc->pid = i + 1; // PID starts from 1
    proc->state = PROC_STATE_RUNNABLE;
    proc->sp = (uint32_t)sp;
    proc->page_table = page_table;
    return proc;
}

void yield(void) {
    // Search the runnable process to run next
    struct process *next = idle_proc;
    for (int i = 0; i < PROCS_MAX; i++) {
        struct process *proc = &procs[(current_proc->pid + i) % PROCS_MAX];
        if (proc->state == PROC_STATE_RUNNABLE && proc->pid > 0) {
            next = proc;
            break;
        }
    }

    // Only the current process is running. No context switch is needed
    if (next == current_proc) {
        return;
    }

    __asm__ __volatile__(
        // Switch the page table by registering VPN[1] to satp register.
        // The sfence.vma instruction is a memory barrier to
        // - ensure modifying the page table finishes
        // - remove the cache of the page table entry (TLB)
        "sfence.vma\n"
        "csrw satp, %[satp]\n"
        "sfence.vma\n"
        // Save the next process's stack bottom address to sscratch register for exception handling.
        "csrw sscratch, %[sscratch]\n"
        :
        : [satp] "r"(SATP_SV32 | ((uint32_t)next->page_table / PAGE_SIZE)),
          [sscratch] "r"((uint32_t)&next->stack[sizeof(next->stack)]));

    // Switch context from the current process to the next runnable process
    struct process *prev = current_proc;
    current_proc = next;
    switch_context(&prev->sp, &next->sp);
}

void proc_init() {
    // The idle process is a special process which runs when there is no runnable process
    current_proc = idle_proc = create_process(NULL, 0);
    idle_proc->pid = -1;
}

__attribute__((noreturn)) void exit() {
    printf("process %d exited\n", current_proc->pid);
    current_proc->state = PROC_STATE_EXITED;
    // TODO: Release process struct instance (kernel stack, page table, ...) since it will no longer used.
    yield();
    PANIC("unreachable");
}
