#include <stdio.h>
#include <stdlib.h>

#include "circular-buffer.h"

/* https://adventofcode.com/2021/day/1#part2 */

#define FALSE 0
#define TRUE 1

#define CIRCULAR_BUFFER_CAPACITY 3

typedef unsigned char BOOL;

BOOL getWindow(FILE* fp, CircularBuffer* cb, int* window);

int main() {
    FILE *fp;
    const char* filename = "input1.txt";
    int previousWindow, currentWindow, numOfIncreases;
    CircularBuffer* cb;

    cb = cb_create(CIRCULAR_BUFFER_CAPACITY);
    previousWindow = -1;
    numOfIncreases = 0;
    
    fp = fopen(filename, "r");
    if (fp == NULL) {
        printf("There was a problem opening %s.", filename);
        exit(1);
    }

    while(getWindow(fp, cb, &currentWindow)) {
        if (previousWindow != -1 && currentWindow > previousWindow) {
            numOfIncreases++;
        }

        cb_print(cb);

        previousWindow = currentWindow;
        currentWindow = 0;
    }

    fclose(fp);
    cb_cleanup(cb);

    printf("---------------------------------\n");
    printf("Number of window increases: %d\n", numOfIncreases);

    return 0;
}

BOOL getWindow(FILE* fp, CircularBuffer* cb, int* windowSum) {
    char line[8];
    int convertedNumber, i = 0;
    BOOL filledAtleastOne = FALSE, shouldContinue = TRUE, initialFill = cb->start == -1;

    while (shouldContinue && fgets(line, sizeof(line), fp)) {
        convertedNumber = atoi(line);
        cb_add(cb, convertedNumber);

        i++;
        filledAtleastOne = TRUE;
        shouldContinue = initialFill && i < 3;
    }

    if (filledAtleastOne) {
        *windowSum = cb_sum(cb);
        return TRUE;
    } else {
        return FALSE;
    }
}