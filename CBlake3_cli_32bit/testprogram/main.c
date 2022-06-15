/* Aplikacia CBlake3 CLI
2022-06-07, TUKE, P.Z.
Aplikacia vyuziva opt. implementaciu hashovacej funkcie Blake3
prostrednictvom DLL kniznice rustblake.dll. */
#include <stdio.h>
#include <inttypes.h>
#include <stdlib.h>
#include "headers/mycrate.h" // import hlavickovych suborov

void usage(void)
{
	printf("USAGE:\n");
    printf("\tBlake3cli.exe [FLAGS] [OPTION]\n");
    printf("EXAMPLE:\n");
    printf("\tBlake3cli.exe -f path_to_file.txt -t 4\n");
    printf("FLAGS:\n");
	printf(" \t-f <Path to file you want to hash.>\n");
	printf(" \t-t <Number of threads (set available amount of threads)>\n");
	exit (8);
}

int main(int argc, char *argv[]){
    // FLAG HANDELING inspired by : https://www.codingunit.com/c-tutorial-command-line-parameter-parsing
    int i=1; // pocitadlo argumentov
    bool IS_FILEPATH = false; // inicializacia - sluzi na verifikaciu (pritomnost) cesty
    bool IS_THREADS_NUM= false; // inicializacia - sluzi na verifikaciu (pritomnost) vlakien
    char *mypath; // inicializacia - cesta k suboru na hasovanie
    int threads; // inicializacia - pocet vlakien
    
    // CLI aplikacia
    while ((i<argc) && (argv[i][0]=='-')){
        switch (argv[i][1])
		{
			case 'f':
                i++;
                mypath=argv[i];
                IS_FILEPATH=true;
				break;

			case 't':
				i++;
                threads=atoi(argv[i]);
                IS_THREADS_NUM=true; 
				break;

			default:
				printf("Wrong Arguments!\n");
				usage();
		}
        i++;
    }
    if (IS_FILEPATH && IS_THREADS_NUM){ // vykonaj hasovanie suboru ak je pritomna cesta k suboru a pocet vlakien
        const MyString *Blake3Hash = Blake3C(threads,mypath); // hashovanie súboru  Blake3C(pocet vlakien, cesta k suboru), navratova hodnota je datova struktura MyString (vid headers/mycrate.h)-> pre zachovanie kompatibility s Rust API musi byt struktura MyString vyjadrena ako konstanta
        const char *blake3_hash_code= Blake3Hash->hash_code; // ziskanie hash. kodu
        float hash_time =  Blake3Hash->hash_time; // cas hashovania v sec
        printf("Blake3 hash: %s\nTime:%f sec\n",blake3_hash_code,hash_time);
	free_Blake3C(Blake3Hash); // zavolaj funkciu na uvolnenie hash kodu z pamate (alokovane rustom)
    }
    else{
        fprintf(stderr,"Not enought arguments parsed!\n");
		usage();
    }
      
	return (0);
}
