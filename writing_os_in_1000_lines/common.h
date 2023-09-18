#pragma once

typedef int bool;
typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned long long uint64_t;
typedef uint32_t size_t;
typedef uint32_t paddr_t; // Physical address type
typedef uint32_t vaddr_t; // Virtual address type

#define true 1
#define false 0
#define NULL ((void *)0)
#define align_up(value, align) __builtin_align_up(value, align) // Get next aligned value
#define is_aligned(value, align) __builtin_is_aligned(value, align)
#define offsetof(type, member) __builtin_offsetof(type, member) // Get struct member byte offset
#define va_list __builtin_va_list
#define va_start __builtin_va_start
#define va_end __builtin_va_end
#define va_arg __builtin_va_arg

#define SYSCALL_PUTCHAR 1

void *memset(void *buf, char c, size_t n);
void *memcpy(void *dst, void const *src, size_t n);
char *strcpy(char *dst, char const *src);
int strcmp(char const *s1, char const *s2);
void printf(char const *fmt, ...);
