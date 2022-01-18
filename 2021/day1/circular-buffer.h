#include <stdlib.h>

typedef struct {
    int* buffer;
    size_t capacity;
    int start;
} CircularBuffer;

CircularBuffer* cb_create(size_t capacity);

void cb_add(CircularBuffer* cb, int number);

void cb_print(CircularBuffer* cb);

int cb_get(CircularBuffer* cb, int index);

int cb_sum(CircularBuffer* cb);

void cb_cleanup(CircularBuffer* cb);

