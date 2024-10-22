use crate::assembler::assembler::parse_code;
use std::fs;
use crate::interpreter::interpreter::run;
use crate::interpreter::state::{RunningState, State};

mod assembler;
mod interpreter;

fn main() {
    let mem = [
        0xf101_u16, 0x0007_u16, 0xf211_u16, 0x0007_u16, 0x1312_u16, 0x1421_u16, 0xc000_u16,
        0x0001_u16, 0x0101_u16,
    ];
    let code = parse_code(&read_from_file("assemble"));
    

    code.iter().for_each(|x| println!("{:#06x}", x));
    let mut state = State::new(&code);
    state.verbose = true;
    state.state = RunningState::Ready;
    run(&mut state);
}

#[must_use] pub fn read_from_file(file: &str) -> String {
    fs::read_to_string(file).expect("File Read")
}

//
// #[wasm_bindgen]
// pub fn assemble_from_file(file: web_sys::) {
//     let mut memory: Vec<u16> = Vec::new();
//
//
// }
