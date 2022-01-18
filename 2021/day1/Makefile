CC=gcc
FLAGS=-std=c89 -Wall -g

cb.o:
	$(CC) $(FLAGS) -c circular-buffer.c -o cb.o

cbmain.o:
	$(CC) $(FLAGS) -c cbmain.c -o cbmain.o

cb: cb.o cbmain.o
	$(CC) $(FLAGS) cb.o cbmain.o -o cb

cbclean:
	rm cb.o cbmain.o cb

main1:
	$(CC) $(FLAGS) main1.c -o main1

main2.o: 
	$(CC) $(FLAGS) -c main2.c

main2: cb.o main2.o
	$(CC) $(FLAGS) cb.o main2.o -o main2

clean:
	rm main1;