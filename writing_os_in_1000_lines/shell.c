#include "user.h"

void main(void) {
    for (;;) {
        printf("> ");
        char cmdline[128];
        uint32_t i = 0;
        while (i <= sizeof(cmdline) - 1) {
            char const c = getchar();
            putchar(c);
            if (c == '\r') { // In QEMU debug console, newlines are \r
                break;
            }
            cmdline[i] = c;
            i++;
        }
        putchar('\n');

        if (i > sizeof(cmdline) - 1) {
            printf("command line too long\n");
            continue;
        }

        cmdline[i] = '\0';

        if (strcmp(cmdline, "hello") == 0) {
            printf("Hello world from shell!\n");
        } else if (strcmp(cmdline, "exit") == 0) {
            exit();
        } else if (strcmp(cmdline, "readfile") == 0) {
            char buf[128];
            int len = readfile("./hello.txt", buf, sizeof(buf));
            buf[len] = '\0';
            printf("%s\n", buf);
        } else if (strcmp(cmdline, "writefile") == 0) {
            writefile("./hello.txt", "Hello from shell!\n", 19);
        } else {
            printf("unknown command: %s\n", cmdline);
        }
    }
}
