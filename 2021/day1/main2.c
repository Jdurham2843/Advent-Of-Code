#include <stdio.h>
#include <stdlib.h>

/* https://adventofcode.com/2021/day/1#part2 */

/* I need to fix my windowing code. I'm currently just getting 3 lines, then the next 3, etc,
   instead of windowing. */

#define FALSE 0
#define TRUE 1

int getWindow(FILE* fp, char** window);

int main() {
    FILE *fp;
    const char* filename = "input1.txt";
    int previousWindow, currentWindow, numOfIncreases;

    previousWindow = -1;
    numOfIncreases = 0;

    fp = fopen(filename, "r");
    if (fp == NULL) {
        printf("There was a problem opening %s.", filename);
        exit(1);
    }

    while(getWindow(fp, &currentWindow)) {
        if (previousWindow != -1 && currentWindow > previousWindow) {
            numOfIncreases++;
        }

        previousWindow = currentWindow;
        currentWindow = 0;
    }

    fclose(fp);

    printf("---------------------------------\n");
    printf("Number of window increases: %d\n", numOfIncreases);

    return 0;
}

int getWindow(FILE* fp, char** window) {
    char line[8];
    int convertedNumber, sum, i = 0;

    while (i < 3 && fgets(line, sizeof(line), fp)) {
        convertedNumber = atoi(line);
        sum += convertedNumber;

        i++;
    }

    if (i == 3) {
        *window = sum;
        return TRUE;
    } else {
        return FALSE;
    }
}