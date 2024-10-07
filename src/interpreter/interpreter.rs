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
    r: [Register;15],
    halt: Register,

    memory: Memory
}

impl State {
    pub fn new(memory: &[u16]) -> State {
        State{
            pc:(Register::new()),
            r: [Register::new(); 15],
            halt: (Register::new()),
            memory: (Memory::new(Option::from(memory))),
        }
    }
}

pub fn init(memory: &[u16]) -> State{
    let mut state: State = State::new(memory);

    state.pc.set(0);

    return state;
}

pub fn step(state: &mut State) {
    let opcode = next_op(&state.memory, &mut state.pc);
    execute(opcode.unwrap(), state);
}

// Set R15 bits for the result of arithmetic operations
fn arith_set_r15(state: &mut State, result: u32, binary: bool, tc: bool) {
    let mut r15: u16 = 0;

    if (result > 0 && binary) { r15 = r15 | R15_G}                      // result > 0 (binary)
    if (result < 0 && binary) { r15 = r15 | R15_V}                      // result < 0 (binary)
    if (((result as u16) & TC_MASK) > 0 && tc) { r15 = r15 | R15_g}     // result > 0 (twos complement)
    if (((result as u16) & TC_MASK) < 0 && tc) { r15 = r15 | R15_lt }   // result < 0 (twos complement)
    if (result == 0) { r15 = r15 | R15_eq}                              // result == 0
    if (((result >> 16) as u16) > 0 ) {
        if (binary) { r15 = r15 | R15_V; }                              // overflow (binary)
        if (tc) { r15 = r15 | R15_v; }                                  // overflow (twos complement)
        r15 = r15 | R15_C;                                              // carry bit set
    }

    state.r[15].set(r15);
}

fn execute(opcode: OpCodes, state: &mut State) {
    match opcode {
        // ================
        // RRR Instructions
        // ================
        OpCodes::Add(..) => {
            println!("Executing add, PC: {}", state.pc.get());
            let mut temp: u32 = state.r[opcode[1]] + state.r[opcode[0]];

            state.r[opcode[2]] = temp as u16;

            // Set R15 bits
            arith_set_r15(state, temp, true, true);
        },
        OpCodes::Addc(..) => {
            println!("Executing addc, PC: {}", state.pc.get());
            // Is carry bit set?
            let carry_set: bool = (state.r[15].get() & R15_C) > 0;
            let mut temp: u32 = state.r[opcode[1]] + state.r[opcode[0]];

            if (carry_set) { temp = temp + 1; }

            state.r[opcode[2]] = temp as u16;

            // Set R15 bits
            arith_set_r15(state, temp, true, true);

        },
        OpCodes::Sub(..) => {
            println!("Executing sub, PC: {}", state.pc.get());
            state.r[opcode[2]] = state.r[opcode[1]] - state.r[opcode[0]];
            arith_set_r15(state, state.r[opcode[2]], true, true);
        },
        OpCodes::Mul(..) => {
            println!("Executing mul, PC: {}", state.pc.get());
            state.r[opcode[2]] = state.r[opcode[1]] * state.r[opcode[0]];
            arith_set_r15(state, state.r[opcode[2]], false, true);
        },
        OpCodes::Muln(..) => {
            println!("Executing muln, PC: {}", state.pc.get());
            let temp: u32 = state.r[opcode[1]] * state.r[opcode[0]];
            state.r[opcode[2]].set(temp as u16);
            state.r[15].set((temp >> 16) as u16);
        },
        OpCodes::Div(..) => {
            println!("Executing div, PC: {}", state.pc.get());
            state.r[opcode[2]] = (state.r[opcode[1]] / state.r[opcode[0]]) as u16;
            state.r[15].set(state.r[opcode[1]] % state.r[opcode[0]])
        },
        OpCodes::Divn(..) => {
            println!("Executing divn, PC: {}", state.pc.get());
            let dividend_mask: u32 = 0xffff0000;

            // The left most 16 bits of dividend is contents of R15
            // The right most 16 bits is Ra
            let mut dividend: u32 = (state.r[15].get() as u32) << 16;
            // left most 16 bits is R15, bitwise or with 0xffff[Ra] where Ra is 2 bytes
            dividend = dividend | ((state.r[opcode[1]] as u32) | dividend_mask);

            let quotient: u32 = dividend / (state.r[opcode[0]] as u32);
            let remainder: u16 = (dividend % state.r[opcode[0]]) as u16;

            state.r[15].set((quotient >> 16) as u16);
            state.r[opcode[2]].set(quotient as u16);
            state.r[opcode[1]].set(remainder);
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
        // ================
        // iRX Instructions
        // ================
    }


}
