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
	EstroByte x = {};
	EstroWord y = {};
	main_bloc_setup:
	x.un = est_get().un;
	y.sn= 1;
	main_bloc_loop:
	x.sn = x.sn - (EstroWord){.sn = 1}.sn;
	est_putd(y);
	y.sn = y.sn * (EstroWord){.sn = 2}.sn;
	if (x.un) goto main_bloc_loop;else goto main_bloc_end;
	main_bloc_end:
	est_putd(y);
	x.un = est_getc().un;
	est_putc(x);
	return (EstroWord){.un = y.un};
}


int main(int argc, const char ** argv){{
    EstroWord out = est_main((EstroWord){.sn = argc},(EstroWord){.ptr = (EstroWord*)argv});
    return out.sn;
}}