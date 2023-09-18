#include "user.h"

extern char __stack_top[];

__attribute__((noreturn)) void exit(void) {
    for (;;) {
    }
}

__attribute__((section(".text.start")))
__attribute__((naked)) void
start(void) {
    // .bss section is not zero-initialized here because kernel did it.
    __asm__ __volatile__(
        "mv sp, %[stack_top]\n"
        "call main\n"
        "call exit\n"
        :
        : [stack_top] "r"(__stack_top));
}

int syscall(int const sysno, int const arg0, int const arg1, int const arg2) {
    // These arguments are stored in the trap frame and will be referred by kernel land
    register int a0 __asm__("a0") = arg0;
    register int a1 __asm__("a1") = arg1;
    register int a2 __asm__("a2") = arg2;
    register int a3 __asm__("a3") = sysno;

    __asm__ __volatile__("ecall"
                         : "=r"(a0)
                         : "r"(a0), "r"(a1), "r"(a2), "r"(a3)
                         : "memory");

    return a0;
}

void putchar(char c) {
    syscall(SYSCALL_PUTCHAR, (int)c, /*unused*/ 0, /*unused*/ 0);
}
