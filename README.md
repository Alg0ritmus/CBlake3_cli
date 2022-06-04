# CBlake3_cli
Jednoduchá CLI aplikácia napísaná v jazyku C, ktorá vrácia odtlačok (hašovací kód) hašovacej funkcie Blake3. Aplikácia využíva optimalizovanú a paralizovateľnú implementáciu hašovacej funkcie Blake3, ktorá je napísaná v jazyku Rust. Mapovanie Rust kódu s jazykom C (viď. zdrojový kód v adresári `src`) umožnuje vytvoriť DLL knižnicu, ktorá je využitá práve v C kóde.

## Použitie:

1) CLI aplikácia sa nachádza v adresári `testprogram/`.
2) Aplikáciu je potrebné preložiť pomocou batch súboru (vhodné pre OS windows). Po úspešnom preklade sa vygeneruje spustiteľný súbor Blake3cli.exe.
3) Spustenie aplikácie: `Blake3cli.exe -f test3.txt -t 8` (-f je prepínač pre cestu k súboru, -t je prepínač na počet vlákien | poradie prepínačov je možné zameniť).
4) Napísal som aj jednoduchý help výpis ktorý je možné spustiť napr. príkazom `Blake3cli.exe -help`.
