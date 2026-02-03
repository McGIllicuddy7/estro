make: test2.s
	gcc test2.s estro-rt.c
run: test2.s
	gcc test2.s estro-rt.c
	./a.out

c: test.c
	gcc test.c estro-rt.c
run-c: test.c
	gcc test.c estro-rt.c
	./a.out

