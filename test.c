#include <stdint.h>
typedef union EstroByte{
    char sn;
    unsigned char un;
} EstroByte;
typedef union EstroWord{
    int64_t sn;
    uint64_t un;
    double db;
    union EstroWord * ptr;
    union EstroByte * byte_ptr;
    union EstroByte * str;
} EstroWord;
EstroWord est_fib(EstroWord x);
EstroWord est_get();
extern EstroByte est_getc();
EstroWord est_main(EstroWord argc,EstroWord argv);
extern void est_putc(EstroWord a);
extern void est_putd(EstroWord a);
extern void est_putf(EstroWord a);

EstroWord est_fib(EstroWord x){
	EstroWord cmp = {};
	EstroWord out = {};
	EstroWord i1 = {};
	EstroWord i2 = {};
	fib_bloc_start:
	cmp.sn = x.sn > (EstroWord){.sn = 1}.sn;
	if (cmp.un) goto fib_bloc_fx;else goto fib_bloc_base;
	fib_bloc_base:
	return (EstroWord){.un = (EstroWord){.sn = 1}.un};
	fib_bloc_fx:
	i1.sn = x.sn - (EstroWord){.sn = 1}.sn;
	i2.sn = x.sn - (EstroWord){.sn = 2}.sn;
	i1.un = est_fib((EstroWord){.un = i1.un}).un;
	i2.un = est_fib((EstroWord){.un = i2.un}).un;
	out.sn = i1.sn + i2.sn;
	return (EstroWord){.un = out.un};
}

EstroWord est_get(){
	get_bloc_start:
	return (EstroWord){.un = (EstroWord){.sn = 10}.un};
}

EstroWord est_main(EstroWord argc,EstroWord argv){
	EstroWord fb = {};
	main_bloc_base:
	fb.sn= 10;
	est_putd((EstroWord){.un = fb.un});
	fb.un = est_fib((EstroWord){.un = fb.un}).un;
	est_putd((EstroWord){.un = fb.un});
	return (EstroWord){.un = (EstroWord){.sn = 0}.un};
}


int main(int argc, const char ** argv){{
    EstroWord out = est_main((EstroWord){.sn = argc},(EstroWord){.ptr = (EstroWord*)argv});
    return out.sn;
}}