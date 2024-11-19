use std::fs;

#[cfg(target_arch = "wasm32")]
use gui::gui::run_app;

mod assembler;
#[cfg(target_arch = "wasm32")]
mod gui;
mod interpreter;
mod tui;

use interpreter::state;

#[cfg(target_arch = "wasm32")]
fn main() {
    run_app();
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // let mem = [
    //     0xf101_u16, 0x0007_u16, 0xf211_u16, 0x0007_u16, 0x1312_u16, 0x1421_u16, 0xc000_u16,
    //     0x0001_u16, 0x0101_u16,
    // ];

    let code = assembler::code::Code::new(read_from_file("assemble"));

    println!("Symbole tablel: {:#?}", code.symbol_table);

    code.memory.iter().for_each(|x| println!("{:#06x}", x));
    let mut state = state::State::new(code.memory.as_slice());
    state.verbose = true;
    state.state = state::RunningState::Ready;
    state.monitor_enable(state::MonitorType::Address(1));
    //interpreter::interpreter::run(&mut state);
}

pub fn read_from_file(file: &str) -> String {
    fs::read_to_string(file).expect("File Read")
}
