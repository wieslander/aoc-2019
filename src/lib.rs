use std::fs;
use std::io::{self, Read};

pub fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Could not read stdin");
    buffer
}

pub fn get_input_from_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

pub mod intcode;
