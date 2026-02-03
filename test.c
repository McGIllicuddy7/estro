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
EstroWord est_get();
extern EstroByte est_getc();
EstroWord est_main(EstroWord argc,EstroWord argv);
extern void est_putc(EstroWord a);
extern void est_putd(EstroWord a);

EstroWord est_get(){
	get_bloc_start:
	return (EstroWord){.un = (EstroWord){.sn = 10}.un};
}

EstroWord est_main(EstroWord argc,EstroWord argv){
	EstroByte x [2] = {};
	EstroByte x1 = {};
	EstroWord y = {};
	main_bloc_setup:
	x1.un = est_get().un;
	y.sn= 1;
	main_bloc_loop:
	x1.un = (unsigned char)*(x.ptr+((EstroWord){.sn = 1})).un;
	x1.sn = x1.sn - (EstroWord){.sn = 1}.sn;
	(*(x.byte_ptr+((EstroWord){.sn = 1}))).un= x1.un;
	y.sn = y.sn * (EstroWord){.sn = 2}.sn;
	x1.un = (unsigned char)*(x.ptr+((EstroWord){.sn = 1})).un;
	est_putd((EstroWord){.un = x1.un});
	if (x1.un) goto main_bloc_loop;else goto main_bloc_end;
	main_bloc_end:
	est_putd((EstroWord){.un = y.un});
	x1.un = est_getc().un;
	est_putc((EstroWord){.un = x1.un});
	return (EstroWord){.un = y.un};
}


int main(int argc, const char ** argv){{
    EstroWord out = est_main((EstroWord){.sn = argc},(EstroWord){.ptr = (EstroWord*)argv});
    return out.sn;
}}