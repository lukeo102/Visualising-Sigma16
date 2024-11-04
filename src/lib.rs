use crate::interpreter::interpreter::run;
use crate::interpreter::state::{RunningState, State};
use std::fs;

mod assembler;
mod interpreter;
pub mod app;
mod code_editor;
mod gui;

pub use crate::app::VisualisingSigma16;

// fn main() {
//     let mem = [
//         0xf101_u16, 0x0007_u16, 0xf211_u16, 0x0007_u16, 0x1312_u16, 0x1421_u16, 0xc000_u16,
//         0x0001_u16, 0x0101_u16,
//     ];
//     let code = parse_code(&read_from_file("assemble"));
//
//
//     code.iter().for_each(|x| println!("{:#06x}", x));
//     let mut state = State::new(&code);
//     state.verbose = true;
//     state.state = RunningState::Ready;
//     run(&mut state);
// }

#[must_use]
pub fn read_from_file(file: &str) -> String {
    fs::read_to_string(file).expect("File Read")
}
