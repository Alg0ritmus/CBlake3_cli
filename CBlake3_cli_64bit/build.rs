extern crate cbindgen;

use std::env;
use std::path::Path;
use cbindgen::{Config, Builder};

fn main() {
    let crate_env = env::var("CARGO_MANIFEST_DIR").unwrap(); // Ziskaj PATH tohot rust projektu ako string (cesta adresara v ktorom sa nachádza manifest -> Cargo.toml)
    let crate_path = Path::new(&crate_env); // string Path na objekt Path
    let config = Config::from_root_or_default(crate_path); // A collection of settings to customize the generated bindings.
    Builder::new().with_crate(crate_path.to_str().unwrap()) // A builder for generating a bindings header.
        .with_config(config)
        .generate()
        .expect("Cannot generate header file!")
        .write_to_file("testprogram/headers/mycrate.h"); // vysledkom je vygenerovanie adresárov a hlavičkových súborov (vo vygenerovanom adresari testprogram/headers/) 
}

// inspirovane kodom z: https://stackoverflow.com/questions/66563760/rust-cdylib-crate-linking-dll-to-c-program-in-windows