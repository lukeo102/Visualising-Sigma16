use crate::assembler::tokens::Tokens;
use crate::interpreter::memory::U16_MAX;
use logos::Logos;
use regex::Regex;
use std::collections::HashMap;

macro_rules! regex {
    ($binder:ident=$pattern:literal) => {
        // static $binder: std::sync::LazyLock<Regex> =
        // std::sync::LazyLock::new(|| Regex::new($pattern).unwrap());
        let $binder = Regex::new($pattern).unwrap();
    };
}

pub fn parse_code(code: &str) -> Vec<u16> {
    let lexer = Tokens::lexer(code);
    let mut assembled: Vec<u16> = Vec::with_capacity(U16_MAX as usize);
    let mut cursor = 0;
    let mut data_inserts: HashMap<String, Vec<u16>> = HashMap::new();
    let mut data_locations: HashMap<String, u16> = HashMap::new();

    for token in lexer {
        println!("{token:?}");
        match token {
            Ok(token) => match token {
                Tokens::RRR(instruction) | Tokens::RR(instruction) | Tokens::IRX(instruction) => {
                    assembled.push(instruction);
                    cursor += 1;
                }
                Tokens::RRRArg(args) => {
                    let reg: u16 = parse_rnargs(args, 3);
                    if assembled[cursor - 1] <= 0xc000_u16 {
                        assembled[cursor - 1] |= reg;
                    } else {
                        println!("RRR arguments found where no RRR instruction exists");
                    }
                }
                Tokens::RRArg(args) => {
                    let reg: u16 = parse_rnargs(args, 2);
                    if assembled[cursor - 1] == 0x4000_u16 {
                        assembled[cursor - 1] |= reg;
                    } else {
                        println!("RR arguments found where no RR instruction exists");
                    }
                }
                Tokens::IRXArg(args) => {
                    let reg: (u16, u16) = parse_irxargs(&args, cursor, &mut data_inserts);
                    if assembled[cursor - 1] >= 0xf000_u16 {
                        assembled[cursor - 1] |= reg.0;
                        assembled.push(reg.1);
                        cursor += 1;
                    } else {
                        println!("IRX arguments found where no IRX instruction exists");
                    }
                }
                Tokens::Data(args) => {
                    regex!(
                        regex = r"(?:(?P<name>[a-zA-Z][a-zA-Z0-9]*) +(?P<data>data) +(?:(?P<var>[a-zA-Z][a-zA-Z0-9]*)|(?P<const>[0-9]+)|(?P<hex>\$[a-fA-F0-9]{4})))|(?:(?P<label>[a-zA-Z][a-zA-Z0-9]*) *\n?)"
                    );

                    let extracted = regex.captures(&args).unwrap();

                    if let Some(value) = extracted.name("data") {
                        // If data
                        let name = extracted["name"].to_string();

                        if let Some(value) = extracted.name("hex") {
                            // Match hex
                            assembled.push(u16::from_str_radix(&value.as_str()[1..], 16).unwrap());
                        } else if let Some(value) = extracted.name("const") {
                            // Match constant
                            assembled.push(value.as_str().parse::<u16>().unwrap());
                        } else if let Some(value) = extracted.name("var") {
                            // Match variable
                            assembled.push(0_u16);

                            data_inserts
                                .entry(value.as_str().parse().unwrap())
                                .or_default()
                                .push(cursor as u16);
                        }
                        data_locations.insert(name, cursor as u16);
                        cursor += 1;
                    } else {
                        // else jump label
                        let name = extracted["label"].to_string();

                        data_locations.insert(name.as_str().parse().unwrap(), cursor as u16);
                    }
                }
                Tokens::Jump(command) => {
                    let (instruction, address) = parse_jump(command, &mut data_inserts, cursor);
                    assembled.push(instruction);
                    assembled.push(address);
                    cursor += 2;
                }
                Tokens::Ignore => {}
            },
            Err(e) => println!("uhh ohh {e:?}"),
        };
    }

    // Add variables
    for (name, dest) in &data_locations {
        println!("Adding {name} vars");
        if let Some(locations) = data_inserts.get(name) {
            for location in locations {
                assembled[*location as usize] = *dest;
            }
            data_inserts.remove(name);
        }
    }

    // Any left over entries in data_inserts should result in error

    assembled.shrink_to_fit();
    assembled
}

fn parse_rnargs(args: String, n: usize) -> u16 {
    let mut arg = 0_u16;

    // Reverse Rd,Ra,Rb then loop over them
    for (i, reg) in args.rsplit(',').enumerate().take(n) {
        arg |= reg[1..].parse::<u16>().unwrap() << (4 * i as u16);
    }

    arg
}

fn parse_irxargs(
    args: &str,
    cursor: usize,
    data_inserts: &mut HashMap<String, Vec<u16>>,
) -> (u16, u16) {
    regex!(
        regex = r"[Rr](?P<rd>[0-9]|1[0-5]),(?:(?P<var_match>[a-zA-Z][a-zA-Z0-9]+)|(?P<cons>[0-9]+)|(?P<hex>\$[a-fA-F0-9]{4}))\[[Rr](?P<disp>[0-9]|1[0-5])]"
    );

    let mut arg = 0_u16;
    let mut addr = 0_u16;
    let extarcted_args = regex.captures(args).unwrap();

    arg |= extarcted_args
        .name("rd")
        .unwrap()
        .as_str()
        .parse::<u16>()
        .unwrap()
        << 8;

    arg |= extarcted_args
        .name("disp")
        .unwrap()
        .as_str()
        .parse::<u16>()
        .unwrap()
        << 4;

    if let Some(cons) = extarcted_args.name("cons") {
        // If constant in addr

        addr = cons.as_str().parse::<u16>().unwrap();
    } else if let Some(var_match) = extarcted_args.name("var_match") {
        // If variable in addr
        let var = var_match.as_str();
        // If variable mapping exists, add to the vec, otherwise create mapping
        data_inserts
            .entry(var.parse().unwrap())
            .or_default()
            .push(cursor as u16);
    } else if let Some(hex) = extarcted_args.name("hex") {
        // If hex in addr
        addr = u16::from_str_radix(&hex.as_str()[1..], 16).unwrap();
    } else {
        println!("Unknown argument");
    }

    (arg, addr)
}

fn parse_jump(
    command: String,
    data_inserts: &mut HashMap<String, Vec<u16>>,
    cursor: usize,
) -> (u16, u16) {
    regex!(
        regex = r"(?P<type>jump(?:[a-zA-Z]{2})?) +(?:(?P<label>[A-z][A-Za-z0-9]*)|(?P<const>\$[A-Fa-f0-9]{4}))(?:\[R(?P<register>[0-9]|1[0-5])])?"
    );

    let extracted_command = regex.captures(&command).unwrap();
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

    if let Some(register) = extracted_command.name("register") {
        instruction |= register.as_str().parse::<u16>().unwrap() << 4;
    }

    let address = if let Some(addr) = extracted_command.name("label") {
        data_inserts
            .entry(addr.as_str().parse().unwrap())
            .or_default()
            .push((cursor + 1) as u16);
        0
    } else if let Some(addr) = extracted_command.name("const") {
        u16::from_str_radix(&addr.as_str()[1..], 16).unwrap()
    } else {
        0
    };

    (instruction, address)
}
