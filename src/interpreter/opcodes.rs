use crate::interpreter::interpreter::{R15_eq, R15_g, R15_L};
use crate::interpreter::memory::{word_to_nibbles, Memory};
use crate::interpreter::register::Register;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum OpCodes {
    // RRR instructions
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Mul(u8, u8, u8),
    Div(u8, u8, u8),
    Addc(u8, u8, u8),
    Muln(u8, u8, u8),
    Divn(u8, u8, u8),
    Rrr1(u8, u8, u8),
    Rrr2(u8, u8, u8),
    Rrr3(u8, u8, u8),
    Rrr4(u8, u8, u8),
    Trap(u8, u8, u8),

    // RR instructions
    Cmp(u8, u8),

    // RX instructions
    Lea(u8, u8, u16),
    Load(u8, u8, u16),
    Store(u8, u8, u16),
    Jump(u8, u8, u16),
    Jumpc(fn(u16) -> bool, u8, u16), // Different jump conditions
    Jal(u8, u8, u16),
    Jumpz(u8, u8, u16),
    Jumpnz(u8, u8, u16),
    Testset(u8, u8, u16),
}

// Pass in slice of current + max possible following.
pub fn next_op(
    memory: &Memory,
    pc: &mut Register,
    verbose: bool,
) -> Result<OpCodes, Box<dyn Error>> {
    let word: u16 = memory[pc.poinc(1).into()];

    if verbose {
        print!("Instruction: {word:#06x}");
    }

    // Extract individual nibbles from the word
    let nibbles = word_to_nibbles(word);

    // While there are panics in here, they are unreachable
    // The assembler ensures any instructions that are inserted, are done so correctly
    // The only case a panic might occur is due to logic errors in the interpreter
    let opcode = match nibbles[3] {
        // iRRR instructions
        0 => Ok(OpCodes::Add(nibbles[2], nibbles[1], nibbles[0])),
        1 => Ok(OpCodes::Sub(nibbles[2], nibbles[1], nibbles[0])),
        2 => Ok(OpCodes::Mul(nibbles[2], nibbles[1], nibbles[0])),
        3 => Ok(OpCodes::Div(nibbles[2], nibbles[1], nibbles[0])),
        5 => Ok(OpCodes::Addc(nibbles[2], nibbles[1], nibbles[0])),
        6 => Ok(OpCodes::Muln(nibbles[2], nibbles[1], nibbles[0])),
        7 => Ok(OpCodes::Divn(nibbles[2], nibbles[1], nibbles[0])),
        8 => Ok(OpCodes::Rrr1(nibbles[2], nibbles[1], nibbles[0])),
        9 => Ok(OpCodes::Rrr2(nibbles[2], nibbles[1], nibbles[0])),
        10 => Ok(OpCodes::Rrr3(nibbles[2], nibbles[1], nibbles[0])),
        11 => Ok(OpCodes::Rrr4(nibbles[2], nibbles[1], nibbles[0])),
        12 => Ok(OpCodes::Trap(nibbles[2], nibbles[1], nibbles[0])),

        // RR Instructions
        4 => Ok(OpCodes::Cmp(nibbles[1], nibbles[0])),

        // iRX instructions
        15 => {
            let word2 = memory[pc.poinc(1) as usize];
            if verbose {
                print!(" {word2:#06x}");
            }
            match nibbles[0] {
                0 => Ok(OpCodes::Lea(nibbles[2], nibbles[1], word2)),
                1 => Ok(OpCodes::Load(nibbles[2], nibbles[1], word2)),
                2 => Ok(OpCodes::Store(nibbles[2], nibbles[1], word2)),
                3 => Ok(OpCodes::Jump(nibbles[2], nibbles[1], word2)),
                4 => match nibbles[2] {
                    0 => Ok(OpCodes::Jumpc(
                        |r15| r15 & (R15_eq | R15_L) > 0,
                        nibbles[1],
                        word2,
                    )), // jumple
                    2 => Ok(OpCodes::Jumpc(|r15| r15 & R15_eq == 0, nibbles[1], word2)), // jumpne
                    4 => Ok(OpCodes::Jumpc(
                        |r15| r15 & (R15_eq | R15_g) > 0,
                        nibbles[1],
                        word2,
                    )), // jumpge
                    _ => panic!("Invalid op code"),
                },
                5 => match nibbles[2] {
                    0 => Ok(OpCodes::Jumpc(|r15| r15 & R15_g > 0, nibbles[1], word2)), // jumpgt
                    2 => Ok(OpCodes::Jumpc(|r15| r15 & R15_eq > 0, nibbles[1], word2)), // jumpeq
                    4 => Ok(OpCodes::Jumpc(|r15| r15 & R15_L > 0, nibbles[1], word2)), // jumplt
                    _ => panic!("Invalid op code"),
                },
                6 => Ok(OpCodes::Jal(nibbles[2], nibbles[1], word2)),
                7 => Ok(OpCodes::Jumpz(nibbles[2], nibbles[1], word2)),
                8 => Ok(OpCodes::Jumpnz(nibbles[2], nibbles[1], word2)),
                9 => Ok(OpCodes::Testset(nibbles[2], nibbles[1], word2)),
                _ => panic!("Invalid op code"),
            }
        }

        // iEXP instructions
        _ => panic!("Invalid op code"),
    };
    if verbose {
        println!();
    }
    opcode
}
