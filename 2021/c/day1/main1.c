#include <stdio.h>
#include <stdlib.h>

/* https://adventofcode.com/2021/day/1 */

int main() {
    FILE* fp;
    const char* filename = "input1.txt";
    int previous, current, numLarger;
    char line[8];

    previous = -1;
    numLarger = 0;

    fp = fopen(filename, "r");

    while(fgets(line, sizeof(line), fp)) {
        current = atoi(line);
        printf("current = %d, previous = %d\n", current, previous);

        if (previous != -1 && current > previous) {
            numLarger++;
        }
        previous = current;
    }

    printf("-------------------------------\n");
    printf("%d instances of larger depths\n", numLarger);

    fclose(fp);

    return 0;
}