a.out: queue.c test.c
	clang -Wall -o $@ queue.c test.c

test: a.out
	./a.out

clean:
	rm -f ./a.out

.PHONY: clean test
