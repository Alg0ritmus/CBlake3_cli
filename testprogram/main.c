#include <stdio.h>
#include <inttypes.h>
#include <stdlib.h>
#include "headers/mycrate.h"

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
    int i=1;
    bool IS_FILEPATH = false;
    bool IS_THREADS_NUM= false;
    char *mypath;
    int threads;
    
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
    if (IS_FILEPATH && IS_THREADS_NUM){
        const MyString *Blake3Hash = Blake3C(threads,mypath);
        const char *blake3_hash_code= Blake3Hash->hash_code;
        float hash_time =  Blake3Hash->hash_time;
        printf("Blake3 hash: %s\nTime:%f sec\n",blake3_hash_code,hash_time);
    }
    else{
        fprintf(stderr,"Not enought arguments parsed!\n");
		usage();
    }
      
	return (0);
}