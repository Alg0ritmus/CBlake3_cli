#![allow(unused)]
use std::fs::File;
use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;

use std::io::prelude::*;
use std::time::Instant;



// Ak chceme vratit string hodnotu, bude to komplikovane... najprv si musime vytvorit strukturu, napr. s nazvom MyString, ktora bude obsahuvat
// premennu hash_code typu *const c_char, ktory je kompatibilny s C-ckom 
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
    // convert i8 to usize, toto je vyzadovane rayon modulom, ktory berie na vstup metody num_thread(:uszie) premennu typu uszie
    let active_threads_usize: usize = threads_num as usize;

    // get Rust string from C string, Ak potrebujeme ziskat Rust retazec z C retazca postupujeme nasledovne
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };
    // Ziskanie hodnoty C retazca -> co je vlastne cesta suboru, ktory chceme odhashovat
    let r_str = c_str.to_str().unwrap();

    // citanie suboru
    let mut file = File::open(r_str).expect("err"); // citanie suboru suboru
    let mut binary_file = Vec::new();
    file.read_to_end(&mut binary_file).expect("err2"); // subor do binarneho tvaru
    
    // inicializacia blake3 hashera
    let mut _hasher = blake3::Hasher::new();
    // nastavenie poctu vlakien pre hashovanie
    rayon::ThreadPoolBuilder::new().num_threads(active_threads_usize).build_global().unwrap(); //nastavenie poctu vlakien pre paral. hasovanie dat

    // inicializacia hash. kodu blake3
    let result_from_update;
    
    let now = Instant::now(); // casovac -> pre meranie

        {   
                _hasher.update_rayon(&binary_file); // paralelne hasovanie dat     
                result_from_update=_hasher.finalize();   
        }
    
    let elapsed = now.elapsed(); // koniec merania
    
    // mozny dodatocny vypis na stdout (informacie o hashovani ako napr pocet pouzitych vlakien, hash. kod, cesta k hashovanemu suboru, cas atd.)
    // println!("Modul: Optimalizovana Rust impl.\nAlgoritmus: Blake3\nSubor:{:?}\nPocet vlakien: {:?}",r_str,threads_num);
    // println!("hash:{:?}\ncas: {:.2?}\n",result_from_update.to_hex(),elapsed);

    // Pre vratenie retazca z Rustu do C kodu potrebujeme naplnit strukturu MyString a vratit ju
    
    // Convert hash. kodu, ktory je typu ArrayByte na typ String (typ String Rustovsky typ, nie je kompatibilny s jazykom C )
    let output : String = result_from_update.to_hex().to_string();

    // naplnenie struktury MyString 
    let boxed = Box::new(MyString {
        // convertnutie typu String na typ *const c_char, ktory je kompatibilny s jazykom C
        hash_code : CString::new(output).unwrap().into_raw(),
        hash_time : elapsed.as_secs_f32(),
    });
    
    // return MyString struktury
    Box::into_raw(boxed)
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