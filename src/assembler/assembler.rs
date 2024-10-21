use std::collections::HashMap;
use logos::Logos;
use regex::Regex;
use stdweb::unstable::TryInto;
use crate::assembler::tokens::Tokens;
use crate::interpreter::memory::U16_MAX;

macro_rules! regex {
    ($binder:ident=$pattern:literal) => {
        static $binder: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| { Regex::new($pattern).unwrap() });
    };
}

pub fn parse_code(code: &str) -> Vec<u16>{
    let mut lexer = Tokens::lexer(code);
    let mut assembled:Vec<u16> = Vec::with_capacity(U16_MAX as usize);
    let mut cursor = 0;
    let mut data: HashMap<String, Vec<u16>> = HashMap::new();
    let mut variables: HashMap<String, u16> = HashMap::new();
    
    while let Some(token) = lexer.next() {
        println!("{:?}", token);
        match token {
            Ok(token) => {
                match token {
                    Tokens::RRR(instruction) => {
                        assembled.push(instruction);
                        cursor += 1;
                    },
                    Tokens::IRX(instruction) => {
                        assembled.push(instruction);
                        cursor += 1;
                    },
                    Tokens::RRRArg(args) => {
                        let reg: u16 = parse_rrrargs(args);
                        if assembled[cursor - 1] < 0xc000_u16 {
                            assembled[cursor - 1] = assembled[cursor - 1] & reg;
                        } else {
                            println!("RRR arguments found where no RRR instruction exists");
                        }
                    },
                    Tokens::IRXArg(args) => {
                    let reg: (u16, u16) = parse_irxargs(&args, cursor, &mut data);
                        if assembled[cursor - 1] < 0xc000_u16 {
                            assembled[cursor - 1] = assembled[cursor - 1] & reg.0;
                            assembled.push(reg.1);
                            cursor += 1;
                        } else {
                            println!("IRX arguments found where no IRX instruction exists");
                        }
                    },
                    Tokens::Data(args) => {
                        regex!(REGEX=r"(?P<name>[a-zA-Z][a-zA-Z0-9]*) +data +(?:(?P<var>[a-zA-Z][a-zA-Z0-9]*)|(?P<const>[0-9]+)|(?P<hex>\$[a-fA-F0-9]{4}))");
                        let extracted = dbg!(REGEX.captures(&args).unwrap());

                        let name = extracted["name"].to_string();
                        if let Some(value) = extracted.name("hex") {
                            assembled.push(u16::from_str_radix(&value.as_str()[1..], 16).unwrap());
                        } else if let Some(value) = extracted.name("const") {
                            assembled.push(value.as_str().parse::<u16>().unwrap());
                        } else if let Some(value) = extracted.name("vat") {
                            assembled.push(0_u16);
                            
                            data.entry(value.as_str().parse().unwrap()).or_default().push(cursor as u16);
                            variables.insert(name, cursor as u16);
                        }
                        cursor += 1;
                    }

                }
            },
            Err(e) => println!("uhh ohh {:?}", e),
        };
    }
    
    // Add variables
    for (name, dest) in variables.iter() {
        for location in data.get(name).unwrap() {
            assembled[*location as usize] = *dest;
        }
    }
    
    assembled.shrink_to_fit();
    assembled
    
}

fn parse_rrrargs(args: String) -> u16{
    let mut arg = 0_u16;
    for (i, reg) in args.rsplit(",").enumerate().take(3) {
        arg = arg & (reg[1..].parse::<u16>().unwrap() << 4 * i as u16);
    }
    arg
}

fn parse_irxargs(args: &str, cursor: usize, vars: &mut HashMap<String, Vec<u16>>) -> (u16, u16) {
    let mut arg = 0_u16;
    let mut addr = 0_u16;
    regex!(REGEX=r"[Rr](?P<rd>[0-9]|1[0-5]),(?:(?P<var_match>[a-zA-Z][a-zA-Z0-9]+)|(?P<cons>[0-9]+)|(?P<hex>\$[a-fA-F0-9]{4}))\[[Rr](?P<disp>[0-9]|1[0-5])]");
    
    let extarcted_args = dbg!(REGEX.captures(&args).unwrap());
    arg &= extarcted_args.name("rd").unwrap().as_str().parse::<u16>().unwrap() << 8;
    arg &= extarcted_args.name("disp").unwrap().as_str().parse::<u16>().unwrap() << 4;

    
    if let Some(cons) = extarcted_args.name("addr_const") {
        addr = cons.as_str().parse::<u16>().unwrap();
    } else if let Some(var_match) = extarcted_args.name("addr_var") {
        let var = var_match.as_str();
        // If variable mapping exists, add to the vec, otherwise create mapping
        vars.entry(var.parse().unwrap()).or_default().push(cursor as u16);
    } else if let Some(hex) = extarcted_args.name("addr_hex") {
        addr = u16::from_str_radix(&hex.as_str()[1..], 16).unwrap();
    }

    (arg, addr)
}
