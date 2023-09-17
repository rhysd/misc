#pragma once

typedef unsigned char uint8_t;
typedef unsigned int uint32_t;
typedef uint32_t size_t;

struct sbiret {
    long error;
    long value;
};
