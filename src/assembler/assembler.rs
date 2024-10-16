use std::fs;
use crate::interpreter::memory::U16_MAX;

pub fn assemble(code: &str) -> [u16; U16_MAX as usize] {
    let mut assembled = [0_u16; U16_MAX as usize];
    let mut cursor: usize = 0;
    let c = code.split("\n");

    c.
    
    assembled
}

pub fn read_from_file(file: &str) {
    let file = fs::read_to_string(file).expect("File Read");
    println!("{}", file);
    assemble(&file);
}