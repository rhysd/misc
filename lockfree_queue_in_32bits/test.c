#include "./queue.h"
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>

#define EXP 11

struct msg_queue {
    // (2^EXP - 1) elements of queue
    _Atomic(char *) slots[1 << EXP];
    _Atomic uint32_t q;
};

void *producer(void *arg) {
    printf("P: Start to send 1000 messages\n");

    struct msg_queue *m = (struct msg_queue *)arg;

    for (int i = 0; i < 1000; i++) {
        printf("P: Sending message %d\n", i);
        int idx;
        do {
            idx = queue_push(&m->q, EXP);
        } while (idx < 0); // note: busy-wait while full
        m->slots[idx] = "hello!";
        queue_push_commit(&m->q);
    }

    printf("P: Done\n");
    return NULL;
}

void *consumer(void *arg) {
    printf("C: Receiving 2000 messages\n");

    struct msg_queue *m = (struct msg_queue *)arg;

    for (int i = 0; i < 2000; i++) {
        printf("C: Receiving message %d\n", i);
        int idx;
        uint32_t save;
        char *msg;
        do {
            do {
                idx = queue_pop(&m->q, EXP, &save);
            } while (idx < 0); // Note: busy-wait while empty
            // Note: Loading message can be relaxed
            // msg = atomic_load_explicit(m->slots + i, memory_order_relaxed)
            msg = m->slots[idx];
        } while (!queue_pop_commit(&m->q, save));
        // Consume the message
        printf("C: Received message %d '%s'\n", i, msg);
    }

    printf("C: Done\n");
    return NULL;
}

int main() {
    // Init
    struct msg_queue m;
    m.q = 0;

    pthread_t pt1;
    pthread_create(&pt1, NULL, producer, &m);

    pthread_t pt2;
    pthread_create(&pt2, NULL, producer, &m);

    pthread_t ct;
    pthread_create(&ct, NULL, consumer, &m);

    pthread_join(pt1, NULL);
    pthread_join(pt2, NULL);
    pthread_join(ct, NULL);

    return 0;
}
