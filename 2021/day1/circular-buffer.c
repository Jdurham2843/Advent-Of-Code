#include <stdio.h>
#include "circular-buffer.h"

CircularBuffer* cb_create(size_t capacity) {
    CircularBuffer* cb = malloc(sizeof(CircularBuffer));
    cb->capacity = capacity;
    cb->buffer = malloc(sizeof(int) * capacity);
    cb->start = -1;

    return cb;
}

void cb_add(CircularBuffer* cb, int number) {
    if (cb->start == -1) {
        cb->buffer[0] = number;
        cb->start = 0;
    } else {
        cb->start = (cb->start + 1) % cb->capacity;
        cb->buffer[cb->start] = number;
    }
}

void cb_print(CircularBuffer* cb) {
    int i;

    if (cb->start == -1) {
        printf("[]\n");
        return;
    }

    printf("[ %d", cb->buffer[cb->start]);
    i = (cb->start + 1) % cb->capacity;
    for (; i != cb->start; i = (i + 1) % cb->capacity) {
        printf(", %d", cb->buffer[i]);
    }
    printf("] || start = %d\n", cb->start);
}

int cb_get(CircularBuffer* cb, int index) {
    if (cb->start == -1) {
        return -1;
    }

    return cb->buffer[(cb->start + index) % cb->capacity];
}

int cb_sum(CircularBuffer* cb) {
    int i, sum;
    if (cb->start == -1) {
        return -1;
    }

    sum = 0;
    for (i = 0; i < cb->capacity; i++) {
        sum += cb->buffer[i];
    }

    return sum;
}

void cb_cleanup(CircularBuffer* cb) {
    free(cb->buffer);
    free(cb);
}
