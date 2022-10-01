# CBlake3_cli
Jednoduchá CLI aplikácia napísaná v jazyku C, ktorá vrácia odtlačok (hašovací kód) hašovacej funkcie Blake3. Aplikácia využíva optimalizovanú a paralizovateľnú implementáciu hašovacej funkcie Blake3, ktorá je napísaná v jazyku Rust. Mapovanie Rust kódu s jazykom C (viď. zdrojový kód v adresári `CBlake3_cli_64bit/src` resp. `CBlake3_cli_32bit/src`) umožnuje vytvoriť DLL knižnicu, ktorá je využitá práve v C kóde.

> Poznámka: zdrojové kódy pre adresáre CBlake3_cli_64bit a CBlake3_cli_32bit sú prakticky totožné, líšia sa len zložkou .cargo (pridané v CBlake3_cli_32bit) viď. docs.pdf.

## Použitie:

1) CLI aplikácia sa nachádza v adresári `CBlake3_cli_64bit/testprogram/` pre 64-bitovú platformu alebo `CBlake3_cli_32bit/testprogram/` pre 32-bitovú platformu (OS Winodws).
2) Aplikáciu je potrebné preložiť pomocou batch súboru `preklad.bat` (vhodné pre OS windows). Po úspešnom preklade sa vygeneruje spustiteľný súbor Blake3cli.exe.
3) Spustenie aplikácie: `Blake3cli.exe -f test3.txt -t 8` (-f je prepínač pre cestu k súboru, -t je prepínač na počet vlákien | poradie prepínačov je možné zameniť).
4) Napísal som aj jednoduchý help výpis ktorý je možné spustiť napr. príkazom `Blake3cli.exe -help`.
<br><br>

----
Viac informácií ohľadom projektu CBlake3_cli (ako napríklad opis princípu mapovanie jazyka Rust s jazykom C) je možné nájsť v stručnej dokumentácii (`docs.pdf`).


## Novinky vo verzií 0.1.1

<ol>

<li>CBlake3_cli je od verzie 0.1.1 schopný hašhovať nie len súbory ale aj reťazce (String).
Ak zadáme neplatnú cestu pre súbor, kt. chceme hašovať, tak CBlake3_cli túto neplatnú cestu  
automatický vyhodnotí ako dáta.  

__Príklad:__ _(hashovanie súboru)_

>valid filepath: `test2.txt`  
>
>príkaz:  
`Blake3cli.exe -f test2.txt -t 8`   
>
> _* zahašuje súbor_

<br>

__Príklad:__ _(hashovanie reťazca č.1)_

>invalid filepath: `tento_subor_neexistuje.txt`  
>
>príkaz:  
`Blake3cli.exe -f tento_subor_neexistuje.txt -t 8`   
>
> _* zahašuje retazec_

<br>

__Príklad:__ _(hashovanie reťazca č.2)_

> retazec: `vstupný reťazec,kt. chcem hašhovat`  
>
>príkaz:  
`Blake3cli.exe -f "vstupný reťazec,kt. chcem hašhovat" -t 8`   
>
> _* zahašuje retazec_

<br>

__Príklad:__ _(hashovanie reťazca v C kóde)_

```
char *myString = "Hello World!"; // data na hasovanie
int threads = 8; // pocet vlákien

const MyString *Blake3Hash = Blake3C(threads,myString);// hašovanie

const char *blake3_hash_code= Blake3Hash->hash_code; 
float hash_time =  Blake3Hash->hash_time; 

printf("Blake3 hash: %s\nTime:%f sec\n",blake3_hash_code,hash_time);

free_Blake3C(Blake3Hash); // uvolnenie pamäte
```
</li>


<li>
Verzia 0.1.1 prináša omnoho menšie DLL knižnice. Zmenšenie veľkosti DLL knižníc je dosiahnuté <a href="https://youtu.be/b2qe3L4BX-Y">konfiguraciou</a> Cargo projektu (viac v Cargo.toml).  

<br>

__Zmena veľkosti 32-bit. DLL knižnice:__

Pôvodná veľkosť (~4700kB) -> Aktuálna veľoksť (~420kB)

__Zmena veľkosti 64-bit. DLL knižnice:__

Pôvodná veľkosť (~4900kB) -> Aktuálna veľoksť (~360kB)

</li>

<li>
Hašovanie súborov. V pôvodnej verzii sa pri hašovaní veľkých súborov správal program nepredvídateľne a častokrát "padal". Príčinou toho bolo preťaženie pamäte RAM, pretože pri hašovaní sa súbor načítal do pamäte celý. Od verzie 0.1.1 sa súbor načitáva do pamäte po blokoch o veľkosti 65536 B, čo by malo zabrániť "padaniu" programu. Nevýhodou je dlhší čas hašovania súboru. Veľkosť bloku, ktorý slúži ako buffer pri čítani súboru je fixne nastavený na hodnotu 65536 B, pričom túto veľkosť môžeme manuálne zmeniť (viď. funkcia <a href="https://github.com/Alg0ritmus/CBlake3_cli/blob/0.1.1/CBlake3_cli_64bit/src/additional/MyHasherStruct.rs#L97">read_buffered_and_hash_from_file</a>), hoci sa to neodporúča.  

Hašovanie reťazcov nepodlieha buffrovaniu, a teda reťazec sa pri hašovaní načíta do pamäte celý.
</li>
</ol>



---
Výslednú DLL knižnicu je možné explicitne zmenšiť o ~100kB, stačí "povedať Rustu", aby za ním upratal OS (toto nastavenie je možné využiť pokiaľ užívateľ "vie čo robí"): 

Viac v sekcii [Unwinding the Stack or Aborting in Response to a Panic](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic)  

Stači pridať týchto pár riadkov do Cargo.toml (napr. pod sekciu `[build-dependencies]`):  

```
[profile.release]  
panic = 'abort'  
```
