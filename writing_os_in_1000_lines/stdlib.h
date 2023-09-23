#pragma once

#include "common.h"

void putchar(char ch);
int getchar(void);
__attribute__((noreturn)) void exit(void);
int readfile(char const *filename, char *buf, int const len);
int writefile(char const *filename, char const *buf, int const len);
