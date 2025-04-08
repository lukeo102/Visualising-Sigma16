use crate::assembler::error::AssemblingError;
use crate::assembler::tokens::Tokens;
use log::{log, Level};
use logos::Logos;
use regex::Regex;
use std::collections::HashMap;
use std::u16;

macro_rules! regex {
    ($binder:ident=$pattern:literal) => {
        // static $binder: std::sync::LazyLock<Regex> =
        // std::sync::LazyLock::new(|| Regex::new($pattern).unwrap());
        let $binder = Regex::new($pattern).unwrap();
    };
}

/// A container to hold Sigma16 code and byte code
pub struct Assembler {
    pub code: String,
    pub assembled: Vec<u16>,
    pub symbol_table: HashMap<String, usize>,
    pub mem_to_code: HashMap<usize, usize>,
    pub errors: Vec<AssemblingError>,
    last_token_processed: Tokens,
    line: usize,
    cursor: usize,
    data_inserts: HashMap<String, Vec<u16>>,
    data_locations: HashMap<String, (u16, usize)>,
    pub registers_used: Vec<usize>,
    pub trap_index: Option<usize>,
}

impl Assembler {
    /// Constructor -- takes the raw Sigma16 code as a parameter
    pub fn new(code: String) -> Assembler {
        Assembler {
            code,
            assembled: Vec::with_capacity(u16::MAX as usize),
            symbol_table: HashMap::new(),
            mem_to_code: HashMap::new(),
            errors: Vec::new(),
            last_token_processed: Tokens::Newline,
            line: 1,
            cursor: 0,
            data_inserts: HashMap::new(),
            data_locations: HashMap::new(),
            registers_used: vec![15],
            trap_index: None,
        }
    }

    /// Assemble the raw Sigma16 code provided to the constructor into byte code
    /// If there is an error, a malformed byte code will be produced in Assembler.assembled
    /// Check for errors by ensuring Assembler.errors is of zero length
    pub fn assemble(&mut self) {
        let code = self.code.clone();
        let lexer = Tokens::lexer(&code);

        // Iterate over each token and process it
        for token in lexer {
            log!(Level::Info, "{token:?}");
            match token {
                Ok(token) => {
                    let valid_token = self.validate_token(token.clone());
                    match valid_token {
                        Ok(_) => {
                            self.parse_token(token.clone());
                            self.last_token_processed = token;
                        }
                        Err(error) => {
                            self.errors.push(error);
                        }
                    }
                }
                Err(e) => {
                    log!(Level::Warn, "uhh ohh {e:?}");
                    self.errors.push(AssemblingError {
                        message: "Unknown token.".to_string(),
                        line: self.line,
                        resolution: "".to_string(),
                    });
                }
            };
        }

        // Add variables
        for (name, (dest, line)) in &self.data_locations {
            println!("Adding {name} vars");
            if let Some(locations) = self.data_inserts.get(name) {
                for location in locations {
                    self.assembled[*location as usize] = *dest;
                }
                self.data_inserts.remove(name);
            } else {
                self.errors.push(AssemblingError {
                    message: "Label is not used.".to_string(),
                    line: *line,
                    resolution: "Either a spelling mistake or it is planned to be used later.\nYou can add a jump to this label after trap R0,R0,R0 to get rid of this error.".to_string()
                })
            }
            self.symbol_table
                .insert(name.clone(), dest.clone() as usize);
        }

        match self.trap_index {
            Some(_) => {}
            None => self.errors.push(AssemblingError {
                message: "No trap instruction, program will never terminate when run.".to_string(),
                line: 0,
                resolution: "Add \"trap R0,R0,R0\" at the end of the program.".to_string(),
            }),
        }
        // Any left over entries in data_inserts should result in error

        self.assembled.shrink_to_fit();
        self.registers_used.sort_unstable();
    }

    /// Verifies that tokens are in the correct order
    /// Records errors where tokens are ordered incorrectly
    /// For example: "add R3,variable[R0]"
    ///     We dont want to compile IRX arguments for a RRR instruction
    fn validate_token(&mut self, token: Tokens) -> Result<Tokens, AssemblingError> {
        match token {
            Tokens::Ignore | Tokens::Newline => Ok(token),
            Tokens::Label(_) => match self.last_token_processed {
                Tokens::Newline => Ok(token),
                _ => Err(AssemblingError {
                    message: "Labels need to be on a new line.".to_string(),
                    line: self.line,
                    resolution: "Put the label on a new line.".to_string(),
                }),
            },
            Tokens::RRRArg(_) => match self.last_token_processed {
                Tokens::RRR(_) => Ok(token),
                _ => Err(AssemblingError {
                    message: "Expected RRR instruction for RRR arguments.".to_string(),
                    line: self.line,
                    resolution: "Either incorrect arguments, or incorrect instruction.".to_string(),
                }),
            },

            Tokens::RRArg(_) => match self.last_token_processed {
                Tokens::RR(_) => Ok(token),
                _ => Err(AssemblingError {
                    message: "Expected RR instruction for RR arguments.".to_string(),
                    line: self.line,
                    resolution: "Either incorrect arguments, or incorrect instruction.".to_string(),
                }),
            },

            Tokens::IRXArg(_) => match self.last_token_processed {
                Tokens::IRX(_) => Ok(token),
                _ => Err(AssemblingError {
                    message: "Expected IRX instruction for IRX arguments.".to_string(),
                    line: self.line,
                    resolution: "Either incorrect arguments, or incorrect instruction.".to_string(),
                }),
            },

            Tokens::RRR(_) | Tokens::RR(_) | Tokens::IRX(_) | Tokens::Jump(_) | Tokens::Data(_) => {
                match self.last_token_processed {
                    Tokens::Newline | Tokens::Label(_) => Ok(token),
                    _ => Err(AssemblingError {
                        message: "Instruction not on a new line.".to_string(),
                        line: self.line,
                        resolution: "Start instruction on a new line".to_string(),
                    }),
                }
            }
        }
    }

    /// Parses tokens into byte code
    /// Tokens are only passed to here if the token did not create an error
    fn parse_token(&mut self, token: Tokens) {
        match token {
            // Base instructions, all instructions are created the same way
            Tokens::RRR(instruction) | Tokens::RR(instruction) | Tokens::IRX(instruction) => {
                self.assembled.push(instruction);
                self.mem_to_code.insert(self.cursor, self.line);
                self.cursor += 1;
            }

            // RRR arguments
            // Gets the arguments and bit wise ors them with the instruction
            Tokens::RRRArg(args) => {
                let reg: u16 = self.parse_rnargs(args, 3);
                self.assembled[self.cursor - 1] |= reg;
                if self.assembled[self.cursor - 1] == 0xc000_u16 {
                    self.trap_index = Some(self.cursor - 1);
                }
            }

            // RR arguments
            // Gets the arguments and bit wise ors them with the instruction
            Tokens::RRArg(args) => {
                let reg: u16 = self.parse_rnargs(args, 2);
                self.assembled[self.cursor - 1] |= reg;
            }

            // IRX arguments
            // Gets the arguments and bit wise ors them with the instruction
            // Pushes the address to the next memory address
            Tokens::IRXArg(args) => {
                let (register, address) = self.parse_irxargs(&args);

                self.assembled[self.cursor - 1] |= register;
                self.assembled.push(address);
                self.mem_to_code.insert(self.cursor, self.line);
                self.cursor += 1;
            }

            // Labels -- anything that goes in the symbol table
            // Not actual instructions or arguments, records the memory location it was
            // created at for later processing
            Tokens::Label(arg) => {
                let label = arg.replace(" ", "");

                self.data_locations
                    .insert(label, (self.cursor as u16, self.line));
            }

            // Data -- anything that is artificially inserted into memory
            // Can be a decimal constant, hexadecimal constant, or a label
            Tokens::Data(args) => {
                regex!(
                    regex = r" +data +(?:(P?<var>[a-zA-Z][a-zA-Z0-9_]*)|(?P<const>[0-9]+)|(?P<hex>\$[a-fA-F0-9]{4}))"
                );

                let extracted = regex.captures(&args).unwrap();

                if let Some(value) = extracted.name("hex") {
                    // Match hex
                    self.assembled
                        .push(u16::from_str_radix(&value.as_str()[1..], 16).unwrap());
                } else if let Some(value) = extracted.name("const") {
                    // Match constant
                    self.assembled.push(value.as_str().parse::<u16>().unwrap());
                } else if let Some(value) = extracted.name("var") {
                    // Match variable
                    self.assembled.push(0_u16);

                    self.data_inserts
                        .entry(value.as_str().parse().unwrap())
                        .or_default()
                        .push(self.cursor as u16);
                }
                //self.data_locations
                //    .insert(name, (self.cursor as u16, self.line));
                self.mem_to_code.insert(self.cursor, self.line);
                self.cursor += 1;
            }

            // Jump instructions
            // Handles all jump instructions
            Tokens::Jump(command) => {
                let (instruction, address) = self.parse_jump(command);
                self.assembled.push(instruction);
                self.assembled.push(address);
                self.mem_to_code.insert(self.cursor, self.line);
                self.cursor += 1;
                self.mem_to_code.insert(self.cursor, self.line);
                self.cursor += 1;
            }

            // Tokens that do not insert into memory
            Tokens::Ignore => {}
            Tokens::Newline => {
                self.line += 1;
            }
        }
    }

    /// Processes RRR and RR instruction arguments
    fn parse_rnargs(&mut self, args: String, n: usize) -> u16 {
        let mut arg = 0_u16;
        let mut temp = 0_u16;

        // Reverse Rd,Ra,Rb or Ra,..[Rb] or Ra,Rb then loop over them
        for (i, reg) in args.rsplit(',').enumerate().take(n) {
            temp = reg[1..].parse::<u16>().unwrap();
            arg |= temp << (4 * i as u16);
            if !self.registers_used.contains(&(temp as usize)) {
                self.registers_used.push(temp as usize);
            }
        }

        arg
    }

    /// Processes IRX instruction arguments
    /// Extracts the arguments then the address
    /// Returns (arguments, address) both as u16
    fn parse_irxargs(&mut self, args: &str) -> (u16, u16) {
        regex!(
            regex = r"[Rr](?P<rd>[0-9]|1[0-5]),(?:(?P<var_match>[a-zA-Z][a-zA-Z0-9]*)|(?P<cons>[0-9]+)|(?P<hex>\$[a-fA-F0-9]{4}))\[[Rr](?P<disp>[0-9]|1[0-5])]"
        );

        let mut arg = 0_u16;
        let mut addr = 0_u16;
        let extarcted_args = regex.captures(args).unwrap();

        let rd = extarcted_args
            .name("rd")
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();
        let disp = extarcted_args
            .name("disp")
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();

        // 0xf{rd}{disp}1
        arg |= rd << 8;
        arg |= disp << 4;

        if !self.registers_used.contains(&(rd as usize)) {
            self.registers_used.push(rd as usize);
        }

        if !self.registers_used.contains(&(disp as usize)) {
            self.registers_used.push(disp as usize);
        }

        if let Some(cons) = extarcted_args.name("cons") {
            // If constant in addr

            addr = cons.as_str().parse::<u16>().unwrap();
        } else if let Some(var_match) = extarcted_args.name("var_match") {
            // If variable in addr
            let var = var_match.as_str();
            // If variable mapping exists, add to the vec, otherwise create mapping
            self.data_inserts
                .entry(var.parse().unwrap())
                .or_default()
                .push(self.cursor as u16);
        } else if let Some(hex) = extarcted_args.name("hex") {
            // If hex in addr
            addr = u16::from_str_radix(&hex.as_str()[1..], 16).unwrap();
        } else {
            // This is unreachable as the token would not have been created if it was reachable
            // This is here to satisfy the Rust compiler
            println!("Unknown argument");
        }

        (arg, addr)
    }

    /// Processes jump instructions
    fn parse_jump(&mut self, command: String) -> (u16, u16) {
        regex!(
            regex = r"(?P<type>jump(?:[a-zA-Z]{2})?) +(?:(?P<label>[A-z][A-Za-z0-9]*)|(?P<const>\$[A-Fa-f0-9]{4}))(?:\[R(?P<register>[0-9]|1[0-5])])?"
        );

        let extracted_command = match regex.captures(&command) {
            None => {
                self.errors.push(AssemblingError {
                    message: "Jump instruction error".to_string(),
                    line: self.line,
                    resolution: "Unknown error".to_string(),
                });
                return (0, 0);
            }
            Some(captures) => captures,
        };
        let mut instruction = match Some(extracted_command.name("type").unwrap().as_str()) {
            Some("jump") => 0xf003_u16,
            Some("jumplt") => 0xf405_u16,
            Some("jumple") => 0xf004_u16,
            Some("jumpeq") => 0xf205_u16,
            Some("jumpne") => 0xf204_u16,
            Some("jumpge") => 0xf404_u16,
            Some("jumpgt") => 0xf005_u16,
            _ => 0_u16,
        };

        // The register in jump arguments is optional, defaults to R0
        if let Some(register) = extracted_command.name("register") {
            instruction |= register.as_str().parse::<u16>().unwrap() << 4;
        }

        let address = if let Some(addr) = extracted_command.name("label") {
            self.data_inserts
                .entry(addr.as_str().parse().unwrap())
                .or_default()
                .push((self.cursor + 1) as u16);
            0
        } else if let Some(addr) = extracted_command.name("const") {
            u16::from_str_radix(&addr.as_str()[1..], 16).unwrap()
        } else {
            0
        };

        (instruction, address)
    }
}
