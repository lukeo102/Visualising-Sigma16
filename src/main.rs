use std::fs;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::interpreter::{interpreter::run, state::State, memory::Memory};
use crate::interpreter::state::RunningState;
use crate::assembler::assembler::{parse_code};


mod interpreter;
mod assembler;

fn main() {
    let mem = [
        0xf101_u16,
        0x0007_u16,
        0xf211_u16,
        0x0007_u16,
        0x1312_u16,
        0x1421_u16,
        0xc000_u16,
        0x0001_u16,
        0x0101_u16,
    ];
    
    parse_code(&*read_from_file("assemble"))
    // let memory: &[u16] = &*assemble(read_from_file("assemble"));
    // memory.iter().for_each(|x| println!("{:#06x}", x));
    // let mut state = State::new(&mem);
    // state.verbose = true;
    // state.state = RunningState::Ready;
    // run(&mut state);

}

pub fn read_from_file(file: &str) -> String {
    fs::read_to_string(file).expect("File Read")
}

//
// #[wasm_bindgen]
// pub fn assemble_from_file(file: web_sys::) {
//     let mut memory: Vec<u16> = Vec::new();
// 
// 
// }
