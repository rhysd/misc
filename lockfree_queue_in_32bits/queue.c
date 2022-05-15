#include "./queue.h"
#include <stdatomic.h>

// 32bit integer is a pair of 16bit integers. The higher is used for tail index and the lower is used for head index.

int queue_pop(_Atomic uint32_t *q, int exp, uint32_t *save) {
    uint32_t r = *save = *q; // "acquire"
    int mask = (1u << exp) - 1;
    int head = r & mask;
    int tail = r >> 16 & mask;
    return head == tail ? -1 : tail; // -1 means queue is empty
}

_Bool queue_pop_commit(_Atomic uint32_t *q, uint32_t save) {
    // CAS to check the saved state is still valid and update the current state with the saved one.
    // When the state is changed by some producers, the CAS operation fails and this function returns false.
    return atomic_compare_exchange_strong(q, &save, save + 0x10000); // Note: It’s harmless if this overflows
}

int queue_push(_Atomic uint32_t *q, int exp) {
    uint32_t r = *q; // "acquire"
    int mask = (1u << exp) - 1;
    int head = r & mask;
    int tail = r >> 16 & mask;
    int next = (head + 1u) & mask;
    if (r & 0x8000) { // avoid overflow on commit
        *q &= ~0x8000;
    }
    return next == tail ? -1 : head; // -1 means queue is full
}

void queue_push_commit(_Atomic uint32_t *q) {
    *q += 1; // "release"
}
