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
    union EstroByte * str;
} EstroWord;
EstroWord est_gen();
EstroWord est_main(EstroWord argc,EstroWord argv);

EstroWord est_gen(){
	gen_bloc_end:
	return (EstroWord){.sn = 10};
}

EstroWord est_main(EstroWord argc,EstroWord argv){
	EstroWord x = {};
	main_bloc_declarations:
	x = est_gen();
	main_bloc_base:
	x.sn = x.sn + (EstroWord){.sn = -1}.sn;
	if (x.un) goto main_bloc_base;else goto main_bloc_end;
	main_bloc_end:
	return x;
}


int main(int argc, const char ** argv){{
    EstroWord out = est_main((EstroWord){.sn = argc},(EstroWord){.ptr = (EstroWord*)argv});
    return out.sn;
}}