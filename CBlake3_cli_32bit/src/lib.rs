#![allow(unused)]
use std::fs::File;
use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;

use std::io::prelude::*;
use std::time::Instant;


use std::io;

use std::fs;

use std::error::Error;
use crate::additional::MyHasherStruct::*;
pub mod additional;





// Ak chceme vratit string hodnotu, bude to komplikovane... najprv si musime vytvorit strukturu, napr. s nazvom MyString, ktora bude obsahovat
// premennu hash_code typu *const c_char, ktory je kompatibilny s C-ckom (kniznica libc)
#[derive(Debug)]
#[repr(C)]
pub struct MyString {
    pub hash_code : *const c_char,
    pub hash_time : f32,
}


// Z C-kodu zavolame funkciu Blake3_rust_to_C(thread_num,s), kde parameter 'thread_num' je cislo a 's' je retazec
// a vrátime pointer na strukturu MyString
#[no_mangle]
pub unsafe extern "C" fn Blake3C(threads_num: i8,s: *const c_char)-> *const MyString{
    // konvertovanie typu i8 na typ usize, toto je vyzadovane rayon modulom, ktory berie na vstup metody num_thread(:uszie) premennu typu uszie
    let active_threads_usize: usize = threads_num as usize;

    // ziskanie Rust string z C string, ak potrebujeme ziskat Rust retazec z C retazca postupujeme nasledovne
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };
    // Ziskanie hodnoty C retazca -> co je vlastne cesta suboru, ktory chceme hashovat
    let r_str = c_str.to_str().unwrap();

    // inicializacia vytvorenej struktury HashData
    let path_or_data = HashData::new(&r_str);
    let mut output: String = String::new(); 

    rayon::ThreadPoolBuilder::new().num_threads(active_threads_usize).build_global().unwrap(); //nastavenie poctu vlakien pre paral. hasovanie dat
    
    let now = Instant::now(); // casovac -> pre meranie

        {   
            output = path_or_data.hash(); 
        }
    
    let elapsed = now.elapsed(); // koniec merania
    
    // mozny dodatocny vypis na stdout (informacie o hashovani ako napr pocet pouzitych vlakien, hash. kod, cesta k hashovanemu suboru, cas atd.)
    // println!("Modul: Optimalizovana Rust impl.\nAlgoritmus: Blake3\nSubor:{:?}\nPocet vlakien: {:?}",r_str,threads_num);
    // println!("hash:{:?}\ncas: {:.2?}\n",result_from_update.to_hex(),elapsed);

    // Pre vratenie retazca z Rustu do C kodu potrebujeme naplnit strukturu MyString a vratit ju
    
    // konvertovanie hash. kodu, ktory je typu ArrayByte na typ String (typ String je "Rustovsky" typ, nie je kompatibilny s jazykom C )


    // naplnenie struktury MyString 
    let boxed = Box::new(MyString {
        // convertnutie typu String na typ *const c_char, ktory je kompatibilny s jazykom C
        hash_code : CString::new(output).unwrap().into_raw(),
        hash_time : elapsed.as_secs_f32(), // pridanie odmeraneho casu (pri merani doby hasovania) v sekundach
    });
    
    // return MyString struktury
    Box::into_raw(boxed)
}

#[no_mangle]
pub unsafe extern "C" fn free_Blake3C(ptr: *const MyString) {
    // uvolniť alokovaný string v ruste -> string je "zivy" iba ak sa nachádza v bloku kodu,
    // t.j. po skoncení funkcie je pamat uvolnena (je dolezite string vlastniť Rustom)
    // cize az zo smernika "vytiahneme" string, tak ho Rust sam vymaze z pamate ak uz nebude "relevantny":
    // slide 45-50/67 v prezentácii: https://speakerdeck.com/dbrgn/calling-rust-from-c-and-java?slide=58
    if ptr.is_null() { return; }
    let ptr = ptr as *mut MyString;
    let my_string_struct: Box<MyString> = Box::from_raw(ptr); // ziskanie struktury MyString
    CString::from_raw(my_string_struct.hash_code as *mut c_char); // prebranie vlastnictva (ownership)
    // premennu hash_time nie je potrebne explicitne vymazat z pamate (aj z dovodu prace bez raw smernika)
    

}


/* 
+------------------------+
| Odporucana literatura: |
+------------------------+
1) 
https://docs.rust-embedded.org/book/interoperability/rust-with-c.html#linking-and-greater-project-context

2)
https://github.com/eqrion/cbindgen/blob/master/docs.md

3)
https://stackoverflow.com/questions/66563760/rust-cdylib-crate-linking-dll-to-c-program-in-windows

4) Ako konvertnut Rust string do jazyka C
https://speakerdeck.com/dbrgn/calling-rust-from-c-and-java?slide=19

5) github-zdrojovy kod- k bodu cislo 4)
https://github.com/dbrgn/candidateparser/blob/master/candidateparser-ffi/src/lib.rs

* Odporucam si precitat/pozriet linky zaradom. 
*/
