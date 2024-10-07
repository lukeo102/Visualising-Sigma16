use std::error::Error;
use crate::interpreter::memory::{word_to_nibbles, Memory};
use crate::interpreter::register::Register;


pub enum OpCodes {
    // iRRR instructions
    Add((u8,u8,u8)),
    Sub((u8,u8,u8)),
    Mul((u8,u8,u8)),
    Div((u8,u8,u8)),
    Cmp((u8,u8,u8)),
    Addc((u8,u8,u8)),
    Muln((u8,u8,u8)),
    Divn((u8,u8,u8)),
    Rrr1((u8,u8,u8)),
    Rrr2((u8,u8,u8)),
    Rrr3((u8,u8,u8)),
    Rrr4((u8,u8,u8)),
    Trap((u8,u8,u8)),

    // iRX instructions
    Lea((u8, u8, u16)),
    Load((u8, u8, u16)),
    Store((u8, u8, u16)),
    Jump((u8, u8, u16)),
    Jumpc0((u8, u8, u16)),
    Jumpc1((u8, u8, u16)),
    Jal((u8, u8, u16)),
    Jumpz((u8, u8, u16)),
    Jumpnz((u8, u8, u16)),
    Testset((u8, u8, u16)),

    // iEXP instructions

}

// Pass in slice of current + max possible following.
pub fn next_op(memory: &Memory, pc: &mut Register) -> Result<OpCodes, Box<dyn Error>> {
    let word: u16 = memory[pc.poinc(1)];

    // Extract individual nibbles from the word
    let nibbles = word_to_nibbles(word);

    match nibbles[3] {
        // iRRR instructions
        0 => Ok(OpCodes::Add((nibbles[2], nibbles[1], nibbles[0]))),
        1 => Ok(OpCodes::Sub((nibbles[2], nibbles[1], nibbles[0]))),
        2 => Ok(OpCodes::Mul((nibbles[2], nibbles[1], nibbles[0]))),
        3 => Ok(OpCodes::Div((nibbles[2], nibbles[1], nibbles[0]))),
        4 => Ok(OpCodes::Cmp((nibbles[2], nibbles[1], nibbles[0]))),
        5 => Ok(OpCodes::Addc((nibbles[2], nibbles[1], nibbles[0]))),
        6 => Ok(OpCodes::Muln((nibbles[2], nibbles[1], nibbles[0]))),
        7 => Ok(OpCodes::Divn((nibbles[2], nibbles[1], nibbles[0]))),
        8 => Ok(OpCodes::Rrr1((nibbles[2], nibbles[1], nibbles[0]))),
        9 => Ok(OpCodes::Rrr2((nibbles[2], nibbles[1], nibbles[0]))),
        10 => Ok(OpCodes::Rrr3((nibbles[2], nibbles[1], nibbles[0]))),
        11 => Ok(OpCodes::Rrr4((nibbles[2], nibbles[1], nibbles[0]))),
        12 => Ok(OpCodes::Trap((nibbles[2], nibbles[1], nibbles[0]))),

        // iRX instructions
        15 => match nibbles[0] {
            0 => Ok(OpCodes::Lea((nibbles[2], nibbles[1], memory[pc.poinc(1)]))),
            1 => Ok(OpCodes::Load((nibbles[2], nibbles[1], memory[pc.poinc(1)]))),
            2 => Ok(OpCodes::Store((nibbles[2], nibbles[1], memory[pc.poinc(1)]))),
            3 => Ok(OpCodes::Jump((nibbles[2], nibbles[1], memory[pc.poinc(1)]))),
            4 => Ok(OpCodes::Jumpc0((nibbles[2], nibbles[1], memory[pc.poinc(1)]))),
            5 => Ok(OpCodes::Jumpc1((nibbles[2], nibbles[1], memory[pc.poinc(1)]))),
            6 => Ok(OpCodes::Jal((nibbles[2], nibbles[1], memory[pc.poinc(1)]))),
            7 => Ok(OpCodes::Jumpz((nibbles[2], nibbles[1], memory[pc.poinc(1)]))),
            8 => Ok(OpCodes::Jumpnz((nibbles[2], nibbles[1], memory[pc.poinc(1)]))),
            11 => Ok(OpCodes::Testset((nibbles[2], nibbles[1], memory[pc.poinc(1)]))),
            _ => {Err(Box::new("Opcodes: Unknown iRX instruction")) }
        }

        // iEXP instructions

        _ => {Err(Box::new("Opcodes: Unknown instruction")) }
    }
}
