#![allow(non_upper_case_globals)]
use crate::interpreter::{
    opcodes::{next_op, OpCodes},
    state::{RunningState, State},
};
use log::{log, Level};

const TC_MASK: u16 = 0b1000_0000_0000_0000;
pub(crate) const R15_g: u16 = 0b1;
const R15_G: u16 = 0b10;
pub(crate) const R15_eq: u16 = 0b100;
pub(crate) const R15_L: u16 = 0b1000;
pub(crate) const R15_lt: u16 = 0b1_0000;
const R15_v: u16 = 0b10_0000;
const R15_V: u16 = 0b100_0000;
const R15_C: u16 = 0b1000_0000;
const R15_S: u16 = 0b1_0000_0000;
const R15_s: u16 = 0b10_0000_0000;
const R15_f: u16 = 0b100_0000_0000;

pub fn step(state: &mut State) {
    let opcode = next_op(&state.memory, &mut state.pc, state.verbose).unwrap();
    log!(Level::Info, "{:?}", opcode);
    execute(opcode, state);
}

fn execute(opcode: OpCodes, state: &mut State) {
    match opcode {
        // ================
        // RRR Instructions
        // ================
        OpCodes::Add(..) => {
            if let OpCodes::Add(rd, ra, rb) = opcode {
                let result: u32 =
                    state.r[ra as usize].get() as u32 + state.r[rb as usize].get() as u32;
                state.r[rd as usize].set(result as u16);

                // Set R15 bits
                let mut r15 = 0_u16;
                if result as u16 & TC_MASK == 0 {
                    r15 |= R15_g;
                }
                if result > 0 {
                    r15 |= R15_G;
                }
                if result as u16 == 0 {
                    r15 |= R15_eq;
                }
                if result as u16 & TC_MASK > 0 {
                    r15 |= R15_lt;
                } // if negative number
                if result & 0xffff_0000 > 0 {
                    r15 = r15 | R15_V | R15_C;
                }
                state.r[15].set(r15);
            }
        }
        OpCodes::Addc(..) => {
            if let OpCodes::Addc(rd, ra, rb) = opcode {
                // Is carry bit set?
                let carry_set: bool = (state.r[15].get() & R15_C) > 0;
                let mut result: u32 =
                    state.r[ra as usize].get() as u32 + state.r[rb as usize].get() as u32;

                if carry_set {
                    result += 1;
                }

                state.r[rd as usize].set(result as u16);
                if state.verbose {
                    println!(
                        "  {} + {} = {} Into R{}",
                        state.r[ra as usize].get(),
                        state.r[rb as usize].get(),
                        result,
                        rd
                    );
                }

                // Set R15 bits
                let mut r15 = 0_u16;
                if result as u16 & TC_MASK == 0 {
                    r15 |= R15_g;
                }
                if result > 0 {
                    r15 |= R15_G;
                }
                if result as u16 == 0 {
                    r15 |= R15_eq;
                }
                if result as u16 & TC_MASK > 0 {
                    r15 |= R15_lt;
                } // if negative number
                if result & 0xffff_0000 > 0 {
                    r15 = r15 | R15_V | R15_C;
                }
                state.r[15].set(r15);
            }
        }
        OpCodes::Sub(..) => {
            if let OpCodes::Sub(rd, ra, rb) = opcode {
                // let mut rd_temp = state.r[rd as usize];
                let result = state.r[ra as usize].get() as u32 - state.r[rb as usize].get() as u32;
                state.r[rd as usize].set(result as u16);
                if state.verbose {
                    println!(
                        "  {} - {} = {} Into R{}",
                        state.r[ra as usize].get(),
                        state.r[rb as usize].get(),
                        result,
                        rd
                    );
                }

                // Set R15 bits
                let mut r15 = 0_u16;
                if result as u16 & TC_MASK == 0 {
                    r15 |= R15_g;
                }
                if result > 0 {
                    r15 |= R15_G;
                }
                if result as u16 == 0 {
                    r15 |= R15_eq;
                }
                if result as u16 & TC_MASK > 0 {
                    r15 |= R15_lt;
                } // if negative number
                if result & 0xffff_0000 > 0 {
                    r15 = r15 | R15_V | R15_C;
                }
                state.r[15].set(r15);
            }
        }
        OpCodes::Mul(..) => {
            if let OpCodes::Mul(rd, ra, rb) = opcode {
                let result = state.r[ra as usize].get() as u32 * state.r[rb as usize].get() as u32;
                if state.verbose {
                    println!(
                        "  {} * {} = {} Into R{}",
                        state.r[ra as usize].get(),
                        state.r[rb as usize].get(),
                        result,
                        rd
                    );
                }

                state.r[rb as usize].set(result as u16);

                // Set R15 bits
                let mut r15 = 0_u16;
                if result & 0xffff_0000 > 0 {
                    r15 |= R15_v;
                }
                state.r[15].set(r15);
            }
        }
        OpCodes::Muln(..) => {
            if let OpCodes::Muln(rd, ra, rb) = opcode {
                let temp: u32 =
                    state.r[ra as usize].get() as u32 * state.r[rd as usize].get() as u32;
                state.r[rb as usize].set(temp as u16);
                state.r[15].set((temp >> 16) as u16);
            }
        }
        OpCodes::Div(..) => {
            if let OpCodes::Div(rd, ra, rb) = opcode {
                let ra_value = state.r[ra as usize].get();
                let rd_value = state.r[rd as usize].get();
                state.r[rb as usize].set(ra_value / rd_value);
                state.r[15].set(ra_value % rd_value);
                if state.verbose {
                    println!(
                        "  {} / {} = {} Into R{}, Remainder {}",
                        state.r[ra as usize].get(),
                        state.r[rb as usize].get(),
                        state.r[rb as usize].get(),
                        rd,
                        state.r[15].get()
                    );
                }
            }
        }
        OpCodes::Divn(..) => {
            if let OpCodes::Divn(rd, ra, rb) = opcode {
                let dividend_mask: u32 = 0xffff_0000;

                // The left most 16 bits of dividend is contents of R15
                // The right most 16 bits is Ra
                let mut dividend: u32 = u32::from(state.r[15].get()) << 16;
                // left most 16 bits is R15, bitwise or with 0xffff[Ra] where Ra is 2 bytes
                dividend |= u32::from(state.r[ra as usize].get()) | dividend_mask;

                let quotient: u32 = dividend / u32::from(state.r[rd as usize].get());
                let remainder: u16 = (dividend % u32::from(state.r[rd as usize].get())) as u16;

                state.r[15].set((quotient >> 16) as u16);
                state.r[rb as usize].set(quotient as u16);
                state.r[ra as usize].set(remainder);
            }
        }
        OpCodes::Cmp(..) => {
            if let OpCodes::Cmp(ra, rd) = opcode {
                let mut r15: u16 = 0;
                println!("R{ra} cmp R{rd}");
                if state.r[ra as usize].get() == state.r[rd as usize].get() {
                    print!("eq");
                    r15 |= R15_eq; // Ra == Rb
                    state.r[15].set(r15);
                } else {
                    if state.r[ra as usize].get() > state.r[rd as usize].get() {
                        print!("G");
                        r15 |= R15_G; // Ra > Rb (binary)
                    } else {
                        print!("lt");
                        r15 |= R15_lt; // Ra < Rb (binary)
                    }

                    // If either Ra or Rd is twos complement,
                    //      then if Ra is bigger than Rd
                    //          then Ra is smaller than Rd
                    //      otherwise, Rd is smaller than Ra
                    // otherwise if Ra is less than Rd
                    //      then Ra is smaller than Rd
                    // otherwise Rd is smaller than Ra
                    if (state.r[ra as usize].get() & TC_MASK) > 0
                        || (state.r[rd as usize].get() & TC_MASK) > 0
                    {
                        if state.r[ra as usize].get() > state.r[rd as usize].get() {
                            print!("L");
                            r15 |= R15_L; // Ra < Rb (twos complement)
                        } else {
                            print!("g");
                            r15 |= R15_g; // Ra > Rb (twos complement)
                        }
                    } else if state.r[ra as usize].get() < state.r[rd as usize].get() {
                        print!("L");
                        r15 |= R15_L; // Ra < Rb (twos complement)
                    } else {
                        print!("g");
                        r15 |= R15_g; // Ra > Rb (twos complement)
                    }
                }
                print!("\n");
                state.r[15].set(r15);
            }
        }
        OpCodes::Rrr1(..) => {}
        OpCodes::Rrr2(..) => {}
        OpCodes::Rrr3(..) => {}
        OpCodes::Rrr4(..) => {}
        OpCodes::Trap(..) => {
            if let OpCodes::Trap(ra, rb, rc) = opcode {
                if ra < 255 {
                    match state.r[ra as usize].get() {
                        0 => {
                            // Hault
                            if state.verbose {
                                println!("  Trap: Hault");
                            }
                            state.state = RunningState::Haulted;
                        }
                        1 => {
                            // Non-blocking read
                            if state.verbose {
                                println!("  Trap: Non-blocking read");
                            }
                        }
                        2 => {
                            // Non-blocking write
                            if state.verbose {
                                println!("  Trap: Non-blocking write");
                            }
                        }
                        3 => {
                            // Blocking read
                            if state.verbose {
                                println!("  Trap: Blocking read");
                            }
                        }
                        4 => {
                            // Breakpoint
                            if state.verbose {
                                println!("  Trap: Breakpoint");
                            }
                        }
                        _ => {
                            //
                            if state.verbose {
                                println!("  Trap: Unknown trap");
                            }
                        }
                    }
                } else {
                    // User defined trap
                }
            }
        }
        // ================
        // iRX Instructions
        // ================
        OpCodes::Lea(..) => {
            if let OpCodes::Lea(dst, disp, v) = opcode {
                let result = (u32::from(v) + state.r[disp as usize].get() as u32) as u16;
                state.r[dst as usize].set(result);
                if state.verbose {
                    log!(
                        Level::Info,
                        "  Load {:#06x} into R{}",
                        state.r[dst as usize].get(),
                        dst
                    );
                }
            }
        }
        OpCodes::Load(..) => {
            if let OpCodes::Load(dst, disp, addr) = opcode {
                let mut temp_addr = (state.r[disp as usize].get() as u32) + u32::from(addr);
                if temp_addr > 65534 {
                    temp_addr -= 65534;
                }
                state.r[dst as usize].set(state.memory[temp_addr as usize]);
                if state.verbose {
                    println!(
                        "  Load {:#06x} from {:#06x} into R{}",
                        state.r[dst as usize].get(),
                        temp_addr,
                        dst
                    );
                }
            }
        }
        OpCodes::Store(..) => {
            if let OpCodes::Store(src, disp, addr) = opcode {
                let dst_addr = addr + state.r[disp as usize].get();
                state.memory[dst_addr as usize] = state.r[src as usize].get();
                if state.verbose {
                    println!(
                        "  Store {:#06x} into {:#06x} from R{}",
                        state.r[src as usize].get(),
                        dst_addr,
                        src
                    );
                }
            }
        }
        OpCodes::Jumpc(..) => {
            if let OpCodes::Jumpc(cond, disp, dest) = opcode {
                if cond(state.r[15].get()) {
                    let mut addr = dest as u32 + state.r[disp as usize].get() as u32;
                    if addr > u16::MAX as u32 {
                        addr -= u16::MAX as u32;
                    }
                    state.pc.set(addr as u16)
                }
            }
        }
        OpCodes::Jump(..) => {
            if let OpCodes::Jump(_, disp, dest) = opcode {
                let mut addr = dest as u32 + state.r[disp as usize].get() as u32;
                if addr > u16::MAX as u32 {
                    addr -= u16::MAX as u32;
                }
                state.pc.set(addr as u16)
            }
        }

        _ => {
            if state.verbose {
                log!(Level::Error, "Unknown instruction. Haulting execution");
            }
            state.state = RunningState::Haulted;
        }
    }
}
