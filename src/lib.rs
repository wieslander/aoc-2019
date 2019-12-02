use std::io::{self, Read};

pub fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Could not read stdin");
    buffer
}

pub mod intcode;
