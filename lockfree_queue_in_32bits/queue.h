#ifndef QUEUE_H_INCLUDED
#define QUEUE_H_INCLUDED

#include <stdbool.h>
#include <stdint.h>

// https://nullprogram.com/blog/2022/05/14/
// Max size of queue is 32767.

// `exp` is base-2 exponent of allocated array. This library assumes the array length is 2^exp.

int queue_pop(_Atomic uint32_t *queue, int exp, uint32_t *save);
_Bool queue_pop_commit(_Atomic uint32_t *queue, uint32_t save);
int queue_push(_Atomic uint32_t *queue, int exp);
void queue_push_commit(_Atomic uint32_t *queue);

#endif
