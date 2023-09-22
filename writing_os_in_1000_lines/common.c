#include "common.h"

void putchar(char ch);

void *memset(void *buf, char c, size_t n) {
    uint8_t *p = (uint8_t *)buf;
    while (n--) {
        *p++ = c;
    }
    return buf;
}

void *memcpy(void *dst, void const *src, size_t n) {
    uint8_t *d = (uint8_t *)dst;
    uint8_t const *s = (uint8_t const *)src;
    while (n--) {
        *d++ = *s++;
    }
    return dst;
}

char *strcpy(char *dst, char const *src) {
    char *d = dst;
    while (*src) {
        *d++ = *src++;
    }
    *d = '\0';
    return dst;
}

int strcmp(char const *s1, char const *s2) {
    while (*s1 && *s2 && *s1 == *s2) {
        s1++;
        s2++;
    }
    return *s1 - *s2;
}

int strlen(char const *s) {
    for (int i = 0;; i++) {
        if (s[i] == '\0') {
            return i;
        }
    }
}

bool startswith(char const *heystack, char const *needle) {
    for (int i = 0;; i++) {
        char const h = heystack[i], n = needle[i];
        if (n == '\0') {
            return true;
        }
        if (h != n) {
            return false;
        }
    }
}

void printf(char const *fmt, ...) {
    va_list vargs;
    va_start(vargs, fmt);

    while (*fmt) {
        if (*fmt == '%') {
            if (*(fmt + 1)) {
                fmt++;
            }
            switch (*fmt) {
            case '%':
                putchar('%');
                break;
            case 's': {
                char const *s = va_arg(vargs, const char *);
                while (*s) {
                    putchar(*s);
                    s++;
                }
                break;
            }
            case 'd': {
                int value = va_arg(vargs, int);
                if (value < 0) {
                    putchar('-');
                    value = -value;
                }

                int divisor = 1;
                while (value / divisor > 9) {
                    divisor *= 10;
                }

                while (divisor > 0) {
                    putchar('0' + value / divisor);
                    value %= divisor;
                    divisor /= 10;
                }

                break;
            }
            case 'x': {
                int value = va_arg(vargs, int);
                for (int i = 7; i >= 0; i--) {
                    int nibble = (value >> (i * 4)) & 0xf;
                    putchar("0123456789abcdef"[nibble]);
                }
            }
            }
        } else {
            putchar(*fmt);
        }

        fmt++;
    }

    va_end(vargs);
}
