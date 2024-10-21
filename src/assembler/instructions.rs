use crate::assembler::instructions::InstructionType::Unknown;

pub enum InstructionType {
    RRR(RRRInstruction),
    IRX(IRXInstruction),
    Unknown
}

impl InstructionType {
    pub(crate) fn from_string(s: &str) -> InstructionType {
        match s {
            "add" => InstructionType::RRR(RRRInstruction::Add),
            "sub" => InstructionType::RRR(RRRInstruction::Sub),
            "mul" => InstructionType::RRR(RRRInstruction::Mul),
            "div" => InstructionType::RRR(RRRInstruction::Div),
            "cmp" => InstructionType::RRR(RRRInstruction::Cmp),
            "addc" => InstructionType::RRR(RRRInstruction::Addc),
            "muln" => InstructionType::RRR(RRRInstruction::Muln),
            "divn" => InstructionType::RRR(RRRInstruction::Divn),
            "rrr1" => InstructionType::RRR(RRRInstruction::Rrr1),
            "rrr2" => InstructionType::RRR(RRRInstruction::Rrr2),
            "rrr3" => InstructionType::RRR(RRRInstruction::Rrr3),
            "rrr4" => InstructionType::RRR(RRRInstruction::Rrr4),
            "trap" => InstructionType::RRR(RRRInstruction::Trap),
            "lea" => InstructionType::IRX(IRXInstruction::Lea),
            "load" => InstructionType::IRX(IRXInstruction::Load),
            "store" => InstructionType::IRX(IRXInstruction::Store),
            "jump" => InstructionType::IRX(IRXInstruction::Jump),
            "jumpc0" => InstructionType::IRX(IRXInstruction::Jumpc0),
            "jumpc1" => InstructionType::IRX(IRXInstruction::Jumpc1),
            "jal" => InstructionType::IRX(IRXInstruction::Jal),
            "jumpz" => InstructionType::IRX(IRXInstruction::Jumpz),
            "jumpnz" => InstructionType::IRX(IRXInstruction::Jumpnz),
            "testset" => InstructionType::IRX(IRXInstruction::Testset),
            
            _ => {Unknown}
        }
    }
    
    pub fn to_string(self) -> String {
        match self { 
            InstructionType::RRR(rrr) => {rrr.to_string()}
            InstructionType::IRX(irx) => {irx.to_string()}
            InstructionType::Unknown => {format!("unknown instruction")}
        }
    }
}

pub enum RRRInstruction {
    Add,
    Sub,
    Mul,
    Div,
    Cmp,
    Addc,
    Muln,
    Divn,
    Rrr1,
    Rrr2,
    Rrr3,
    Rrr4,
    Trap,
}

impl RRRInstruction {
    pub fn to_int(&self) -> u16 {
        match self {
            RRRInstruction::Add => 0x0,
            RRRInstruction::Sub => 0x1,
            RRRInstruction::Mul => 0x2,
            RRRInstruction::Div => 0x3,
            RRRInstruction::Cmp => 0x4,
            RRRInstruction::Addc => 0x5,
            RRRInstruction::Muln => 0x6,
            RRRInstruction::Divn => 0x7,
            RRRInstruction::Rrr1 => 0x8,
            RRRInstruction::Rrr2 => 0x9,
            RRRInstruction::Rrr3 => 0xa,
            RRRInstruction::Rrr4 => 0xb,
            RRRInstruction::Trap => 0xc,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self { 
            RRRInstruction::Add => String::from("ADD"),
            RRRInstruction::Sub => String::from("SUB"),
            RRRInstruction::Mul => String::from("MUL"),
            RRRInstruction::Div => String::from("DIV"),
            RRRInstruction::Cmp => String::from("CMP"),
            RRRInstruction::Addc => String::from("ADDC"),
            RRRInstruction::Muln => String::from("MULN"),
            RRRInstruction::Divn => String::from("DIVN"),
            RRRInstruction::Rrr1 => String::from("RR1"),
            RRRInstruction::Rrr2 => String::from("RR2"),
            RRRInstruction::Rrr3 => String::from("RR3"),
            RRRInstruction::Rrr4 => String::from("RR4"),
            RRRInstruction::Trap => String::from("TRAP"),
        }
    }
}

pub enum IRXInstruction {
    Lea,
    Load,
    Store,
    Jump,
    Jumpc0,
    Jumpc1,
    Jal,
    Jumpz,
    Jumpnz,
    Testset
}

impl IRXInstruction {
    pub fn to_int(&self) -> u16 {
        match self {
            IRXInstruction::Lea => 0xf000_u16,
            IRXInstruction::Load => 0xf001_u16,
            IRXInstruction::Store => 0x2,
            IRXInstruction::Jump => 0x3,
            IRXInstruction::Jumpc0 => 0x4,
            IRXInstruction::Jumpc1 => 0x5,
            IRXInstruction::Jal => 0x6,
            IRXInstruction::Jumpz => 0x7,
            IRXInstruction::Jumpnz => 0x8,
            IRXInstruction::Testset => 0xb,
        }
    }
    
    pub fn to_string(&self) -> String {
        match self { 
            IRXInstruction::Lea => String::from("LEA"),
            IRXInstruction::Load => String::from("LOAD"),
            IRXInstruction::Store => String::from("STORE"),
            IRXInstruction::Jump => String::from("JUMP"),
            IRXInstruction::Jumpc0 => String::from("JUMPC0"),
            IRXInstruction::Jumpc1 => String::from("JUMPC1"),
            IRXInstruction::Jal => String::from("JAL"),
            IRXInstruction::Jumpz => String::from("JUMPZ"),
            IRXInstruction::Jumpnz => String::from("JUMPNZ"),
            IRXInstruction::Testset => String::from("TESTSET"),
        }
    }
}