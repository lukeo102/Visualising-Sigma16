use crate::interpreter::{opcodes::{OpCodes, next_op}, register::Register, memory::Memory};


const TC_MASK: u16 = 0b1000000000000000;
const R15_g: u16 = 0b1;
const R15_G: u16 = 0b10;
const R15_eq: u16 = 0b100;
const R15_L: u16 = 0b1000;
const R15_lt: u16 = 0b10000;
const R15_v: u16 = 0b100000;
const R15_V: u16 = 0b1000000;
const R15_C: u16 = 0b10000000;
const R15_S: u16 = 0b100000000;
const R15_s: u16 = 0b1000000000;
const R15_f: u16 = 0b10000000000;

pub struct State {
    pc: Register,
    r: [Register],
    halt: Register,

    memory: Memory
}

pub fn step() {

}

pub fn init() {

}

fn arith_set_r15(state: &mut State, result: u32) {
    let mut r15: u16 = 0;

    if (result > 0) { r15 = r15 | R15_G}                        // result > 0 (binary)
    if (result < 0) { r15 = r15 | R15_V}                        // result < 0 (binary)
    if (((result as u16) & TC_MASK) > 0) { r15 = r15 | R15_g}   // result > 0 (twos complement)
    if (((result as u16) & TC_MASK) < 0) { r15 = r15 | R15_lt } // result < 0 (twos complement)
    if (result == 0) { r15 = r15 | R15_eq}                      // result == 0
    if (((result >> 16) as u16) > 0 ) {
        r15 = r15 | R15_V;                                      // overflow (binary)
        r15 = r15 | R15_v;                                      // overflow (twos complement)
        r15 = r15 | R15_C;                                      // carry bit set
    }

    state.r[15].set(r15);
}

fn execute(opcode: OpCodes, state: &mut State) {
    match opcode {

        // Normal addition
        OpCodes::Add(..) => {
            println!("Executing add, PC: {}", state.pc.get());
            let mut temp: u32 = state.r[opcode[1]] + state.r[opcode[0]];
            let mut r15: u16 = 0;

            state.r[opcode[2]] = temp as u16;

            // Set R15 bits
            arith_set_r15(state, temp);
        },

        // Add with carry
        OpCodes::Addc(..) => {
            println!("Executing addc, PC: {}", state.pc.get());
            // Is carry bit set?
            let carry_set: bool = (state.r[15].get() & R15_C) > 0;
            let mut temp: u32 = state.r[opcode[1]] + state.r[opcode[0]];

            if (carry_set) { temp = temp + 1; }

            state.r[opcode[2]] = temp as u16;

            // If overflow, record it in R15
            if (((temp >> 16) as u16) > 0 ) { state.r[15].set(0b11100010) }

        },

        // Normal Subtraction
        OpCodes::Sub(..) => {
            println!("Executing sub, PC: {}", state.pc.get());
            state.r[opcode[2]] = state.r[opcode[1]] - state.r[opcode[0]];
            arith_set_r15(state, state.r[opcode[2]]);
        },
        OpCodes::Mul(..) => {
            println!("Executing mul, PC: {}", state.pc.get());
            state.r[opcode[2]] = state.r[opcode[1]] * state.r[opcode[0]];
            arith_set_r15(state, state.r[opcode[2]]);
        },
        OpCodes::Muln(..) => {
            println!("Executing muln, PC: {}", state.pc.get());
        },
        OpCodes::Div(..) => {
            println!("Executing div, PC: {}", state.pc.get());
            state.r[opcode[2]] = (state.r[opcode[1]] / state.r[opcode[0]]) as u16;
            state.r[15].set(state.r[opcode[1]] % state.r[opcode[0]])
        },
        OpCodes::Divn(..) => {
            println!("Executing divn, PC: {}", state.pc.get());
        },
        OpCodes::Cmp(..) => {
            println!("Executing cmp, PC: {}", state.pc.get());
            let mut r15: u16 = 0;

            if (state.r[opcode[1]] == state.r[opcode[0]]) {
                r15 = r15 | 0b100;                                                      // Ra == Rb
                state.r[15].set(r15);
            } else {
                if (state.r[opcode[1]] > state.r[opcode[0]]) { r15 = r15 | 0b10 }       // Ra > Rb (binary)
                else { r15 = r15 | 0b10000 }                                            // Ra < Rb (binary)
                if ((state.r[opcode[1]] & TC_MASK) > 0 || (state.r[opcode[0]] & TC_MASK) > 0) {
                    if {state.r[opcode[1]] > state.r[opcode[0]]} {r15 = r15 | 0b1000}   // Ra < Rb (twos complement)
                    else { r15 = r15 | 0b1 }                                            // Ra > Rb (twos complement)
                }
            }
        },
        OpCodes::Rrr1(..) => {
            println!("Executing rrr1, PC: {}", state.pc.get());
        },
        OpCodes::Rrr2(..) => {
            println!("Executing rrr2, PC: {}", state.pc.get());
        },
        OpCodes::Rrr3(..) => {
            println!("Executing rrr3, PC: {}", state.pc.get());
        },
        OpCodes::Rrr4(..) => {
            println!("Executing rrr4, PC: {}", state.pc.get());
        },
        OpCodes::Trap(..) => {
            println!("Executing trap, PC: {}", state.pc.get());
        },

    }


}
