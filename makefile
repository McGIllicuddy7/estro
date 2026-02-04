make: test2.s
	clang test2.s estro-rt.c
run: test2.s
	clang test2.s estro-rt.c
	./a.out

c: test.c
	clang test.c estro-rt.c
run-c: test.c
	clang test.c estro-rt.c
	./a.out

