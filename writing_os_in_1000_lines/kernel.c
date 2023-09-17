#include "kernel.h"
#include "common.h"

extern char __bss[], __bss_end[], __stack_top[], __free_ram[], __free_ram_end[];

struct process procs[PROCS_MAX]; // Note: All elements are zero-initialized so the state is PROC_STATE_UNUSED

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

__attribute__((naked)) void switch_context(uint32_t *prev_sp, uint32_t *next_sp) {
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

struct process *create_process(uint32_t const pc) {
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
    *--sp = 0;  // s11
    *--sp = 0;  // s10
    *--sp = 0;  // s9
    *--sp = 0;  // s8
    *--sp = 0;  // s7
    *--sp = 0;  // s6
    *--sp = 0;  // s5
    *--sp = 0;  // s4
    *--sp = 0;  // s3
    *--sp = 0;  // s2
    *--sp = 0;  // s1
    *--sp = 0;  // s0
    *--sp = pc; // ra

    proc->pid = i + 1; // PID starts from 1
    proc->state = PROC_STATE_RUNNABLE;
    proc->sp = (uint32_t)sp;
    return proc;
}

struct process *current_proc; // The process currently running
struct process *idle_proc;    // The idle process (PID=-1) which runs when no process is runnable

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

    // Save the next process's stack bottom address to sscratch register for exception handling.
    __asm__ __volatile__(
        "csrw sscratch, %[sscratch]\n"
        :
        : [sscratch] "r"((uint32_t)&next->stack[sizeof(next->stack)]));

    // Switch context from the current process to the next runnable process
    struct process *prev = current_proc;
    current_proc = next;
    switch_context(&prev->sp, &next->sp);
}

struct process *proc_a;
struct process *proc_b;

void kernel_main(void) {
    // There is no guarantee that bootloader initializes bss with zeros
    memset(__bss, 0, (size_t)__bss_end - (size_t)__bss);

    // Set exception handler
    WRITE_CSR(stvec, (uint32_t)exception_handler);

    // The idle process is a special process which runs when there is no runnable process
    current_proc = idle_proc = create_process((uint32_t)NULL);
    idle_proc->pid = -1;

    yield();
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

void handle_trap(struct trap_frame *f) {
    (void)f;

    uint32_t scause, stval, user_pc;
    READ_CSR(scause, scause);
    READ_CSR(stval, stval);
    READ_CSR(user_pc, sepc);

    PANIC("unexpected trap scause=%x, stval=%x, sepc=%x\n", scause, stval, user_pc);
}
