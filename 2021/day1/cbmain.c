#include <stdio.h>

#include "circular-buffer.h"

#define CAPACITY 3

int main() {
    int i;
    CircularBuffer* cb = cb_create(CAPACITY);

    cb_print(cb);
    for (i = 0; i < CAPACITY + 5; i++) {
        cb_add(cb, i + 1);
        cb_print(cb);
    }

    printf("index 1 of buffer is %d\n", cb_get(cb, 1));
    printf("sum of the buffer is %d\n", cb_sum(cb));

    cb_cleanup(cb);

    return 0;
}