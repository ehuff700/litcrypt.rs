extern crate rand;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use rand::Rng;


fn generate_xor_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    let mut rng = rand::thread_rng();
    rng.fill(&mut key);
    key
}

fn write_constants() {
    let constants_rs = Path::new(&std::env::var("OUT_DIR").unwrap()).join("constants.rs");

    let xor_key = generate_xor_key();
    let xor_key_str = format!("const XOR_KEY: [u8; 32] = {:?};", xor_key);

    let mut f = File::create(constants_rs).unwrap();
    f.write_all(xor_key_str.as_bytes()).unwrap();
}

fn main() {
    write_constants();
}