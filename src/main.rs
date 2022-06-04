// Autor: Patrik Zeleňák
// Verzia: 0.1
// Poznámka: Následujúci kód bol vytvorený
// v rámci bakalárskej práce - Kryptografia v Pythone
#![allow(unused)] // vypnutie varovani o nevyuzitych premennych a funkciach
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use rayon::prelude::*;
use std::env;


fn main(){
    // citanie stdin parametrov
    let args: Vec<String> = env::args().collect();
    let filename = &args[&args.len()-2];
    let active_threads = &args[&args.len()-1];
    let active_threads_usize: usize = active_threads.parse().unwrap();
    let mut file = File::open(filename).expect("err"); // citanie suboru suboru
    let mut binary_file = Vec::new();
    file.read_to_end(&mut binary_file).expect("err2"); // subor do binarneho tvaru
    
    let mut _hasher = blake3::Hasher::new();
    rayon::ThreadPoolBuilder::new().num_threads(active_threads_usize).build_global().unwrap(); //nastavenie poctu vlakien pre paral. hasovanie dat


   
  
    let result_from_update;
    
    let now = Instant::now(); // casovac

        {   
                _hasher.update_rayon(&binary_file); // paralelne hasovanie dat     
                result_from_update=_hasher.finalize();   
        }
    
    let elapsed = now.elapsed();
    
    
    println!("Modul: Optimalizovana Rust impl.\nAlgoritmus: Blake3\nSubor:{:?}\nPocet vlakien: {:?}",filename,active_threads);
    
    println!("hash:{:?}\ncas: {:.2?}\n",result_from_update.to_hex(),elapsed);
    
}