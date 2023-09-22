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
        } else if (startswith(cmdline, "readfile ")) {
            char buf[128];

            char const *filename = cmdline + 9;
            if (*filename == '\0') {
                printf("empty file name: %s\n", filename);
                continue;
            }

            int const len = readfile(filename, buf, sizeof(buf));
            if (len < 0) {
                printf("file does not exist: %s\n", filename);
                continue;
            }
            buf[len] = '\0';
            printf("%s:\n%s\n", filename, buf);
        } else if (startswith(cmdline, "writefile ")) {
            char *filename = cmdline + 10;
            if (*filename == '\0') {
                printf("empty file name: %s\n", filename);
                continue;
            }

            char *data = filename;
            while (*data != ' ' && *data != '\0') {
                data++;
            }
            if (*data == ' ') {
                *data++ = '\0';
            }

            int const len = writefile(filename, data, strlen(data));
            if (len < 0) {
                printf("file does not exist: %s\n", filename);
            }
        } else {
            printf("unknown command: %s\n", cmdline);
        }
    }
}
