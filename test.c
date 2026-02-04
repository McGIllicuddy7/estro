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
	EstroWord i = {};
	EstroWord cmp = {};
	main_bloc_begin:
	main_bloc_loop:
	i.sn = i.sn + (EstroWord){.sn = 1}.sn;
	est_putd((EstroWord){.un = i.un});
	cmp.sn = i.sn != (EstroWord){.sn = 10}.sn;
	if (cmp.un) goto main_bloc_loop;else goto main_bloc_done;
	main_bloc_done:
	return (EstroWord){.un = i.un};
}


int main(int argc, const char ** argv){{
    EstroWord out = est_main((EstroWord){.sn = argc},(EstroWord){.ptr = (EstroWord*)argv});
    return out.sn;
}}