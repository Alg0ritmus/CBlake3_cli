use std::io;
use std::io::prelude::*;
use std::fs;
use std::env;
use std::fs::File;
use std::error::Error;
use crate::HashData::{PathToFile,DataToHash};


// ****************************************************
// Adding Some code to accept FilePath and hash that file
// Adding Some code to accept Data and hash that data-stream




//////////////////////////////////////
// All functions / methonds should return Result
//


#[derive(Debug)]
pub enum HashData {
    PathToFile(String),
    DataToHash(String)

}

// for now, if path is not find, we will hash that path
// e.g. ./filename.txt -> if "./filename.txt" is not valid,
// we'll perform hash("./filename.txt")
impl <'a>HashData{
    pub fn new (s : &'a str) -> HashData {
        match fs::canonicalize(s) {
            Ok(path) => {
                HashData::PathToFile(s.to_string())
            },
            Err(_)  => HashData::DataToHash(s.to_string())
    
        }
    }
}


pub trait HashMe{
    fn hash(self)-> String;
}

impl HashMe for HashData{

    fn hash(self)-> String{
        
        let hash = match self {
            // hash file by path
            PathToFile(path)  => {
                println!("Hashing file");
                read_buffered_and_hash_from_file(&path)
            },

            // hash data
            DataToHash(data)=> {
                println!("Hashing data");
                read_buffered_and_hash_from_bytes(&data)
            },

        };

        match hash{
            Ok(n) =>n,
            Err(msg) => format!("Error msg: {msg}")
        }



    }

    
}


pub fn read_buffered_and_hash_from_bytes(buffer: &str )->Result<String, Box<dyn Error>>{
    let mut hasher = blake3::Hasher::new();
    let mut buffer = buffer.as_bytes();
   
    hasher.update_rayon(&buffer);
    let mut result = hasher.finalize();
    Ok(result.to_hex().to_string())

}

// Box<dyn Error>  basically return any Type of error
pub fn read_buffered_and_hash_from_file(path: &str)->Result<String, Box<dyn Error>>{
    let mut hasher = blake3::Hasher::new();
    // windows stack buffer is ~1MB, try to keep buffer size at 65 536, which should be save
    // but might be slow. You can experiment with buffer size to increase speed of reading file
    // If size of buffer exceeds stack size, program will panic.
    let mut buffer = [0; 65536]; // was [0; 65536]; 
    let mut file = File::open(path)?;

    loop {
        match file.read(&mut buffer){
            Ok(0) => {
                return Ok(hasher.finalize().to_hex().to_string());
            },
            Ok(n) => {
                //println!("buffer {:?}",&buffer[..n]);
                hasher.update_rayon(&buffer[..n]);
            },
            Err(v) => {
                return Err(Box::new(v));
        }
    
        }

    }
   

}

// ****************************************************