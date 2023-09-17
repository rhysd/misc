#include "kernel.h"
#include "common.h"

extern char __bss[], __bss_end[], __stack_top[];

void *memset(void *buf, char c, size_t n) {
    uint8_t *p = (uint8_t *)buf;
    while (n--) {
        *p++ = c;
    }
    return buf;
}

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

void kernel_main(void) {
    // There is no guarantee that bootloader initializes bss with zeros
    memset(__bss, 0, (size_t)__bss_end - (size_t)__bss);

    printf("\n\nHello %s\n", "World!");
    printf("%x == %d\n", 0x1234abcd, 0x1234abcd);

    for (;;)
        ;
}

__attribute__((section(".text.boot")))
__attribute__((naked)) /* Do not generate prologue and epilogue */
void
boot(void) {
    __asm__ __volatile__(
        "mv sp, %[stack_top]\n"
        "j kernel_main\n"
        :
        : [stack_top] "r"(__stack_top));
}
