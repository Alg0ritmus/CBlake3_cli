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


---
Výslednú DLL knižnicu je možné explicitne zmenšiť o ~100kB, stačí "povedať Rustu", aby za ním upratal OS: 

Viac v sekcii [Unwinding the Stack or Aborting in Response to a Panic](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic)  

Stači pridať týchto pár riadkov do Cargo.toml (napr. pod sekciu `[build-dependencies]`):  

```
[profile.release]  
panic = 'abort'  
```
