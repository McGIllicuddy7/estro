#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
void est_putd(int64_t i){
	printf("%lld\n", i);
}

void est_putc(int64_t c){
	printf("%c\n", (char)(c));
}

char est_getc(){
	return getchar();	
}

void est_putf(double i){
	printf("%lf\n", i);
}

