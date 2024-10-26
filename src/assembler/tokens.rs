use logos::{Logos, Skip};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[\t\r\n\f]+")]
#[logos(skip r" *;.*")]
pub enum Tokens {
    // Non-Instructions
    #[regex(" +", |_| Skip)]
    Ignore,
    #[regex(r"(?:[a-zA-Z][a-zA-Z0-9_]*) *\n?", |lex| lex.slice().to_owned())]
    #[regex(r"(?:[a-zA-Z][a-zA-Z0-9_]*) +data +(?:[a-zA-Z][a-zA-Z0-9_]*|[0-9]+|\$[a-fA-F0-9]{4})"gm, |lex| lex.slice().to_owned())]
    Data(String),
    #[regex(r"[Rr][(?:[0-9])(?:1[0-5])],[Rr][(?:[0-9])(?:1[0-5])],[Rr][(?:[0-9])(?:1[0-5])]", |lex| lex.slice().to_owned())]
    RRRArg(String),
    #[regex(r"[Rr][(?:[0-9])(?:1[0-5])],[Rr][(?:[0-9])(?:1[0-5])]", |lex| lex.slice().to_owned())]
    RRArg(String),
    #[regex(r"[Rr](?:[0-9]|1[0-5]),(?:[a-zA-Z][a-zA-Z0-9_]+|[0-9]+|\$[a-fA-F0-9]{4})\[[Rr](?:[0-9]|1[0-5])]", |lex| lex.slice().to_owned())]
    IRXArg(String),

    // RRR Instructions
    #[regex(r" +add +", |_| 0x0000_u16)]
    #[regex(r" +sub +", |_| 0x1000_u16)]
    #[regex(r" +mul +", |_| 0x2000_u16)]
    #[regex(r" +div +", |_| 0x3000_u16)]
    #[regex(r" +addc +", |_| 0x5000_u16)]
    #[regex(r" +muln +", |_| 0x6000_u16)]
    #[regex(r" +divn +", |_| 0x7000_u16)]
    #[regex(r" +rrr1 +", |_| 0x8000_u16)]
    #[regex(r" +rrr2 +", |_| 0x9000_u16)]
    #[regex(r" +rrr3 +", |_| 0xa000_u16)]
    #[regex(r" +rrr4 +", |_| 0xb000_u16)]
    #[regex(r" +trap +", |_| 0xc000_u16)]
    RRR(u16),
    
    // RR Instructions
    #[regex(r" +cmp +", |_| 0x4000_u16)]
    RR(u16),

    // iRX Instructions
    #[regex(r" +lea +", |_| 0xf000_u16)]
    #[regex(r" +load +", |_| 0xf001_u16)]
    #[regex(r" +store +", |_| 0xf002_u16)]
    #[regex(r" +jal +", |_| 0xf006_u16)]
    #[regex(r" +jumpnz +", |_| 0xf007_u16)]
    #[regex(r" +jumpz +", |_| 0xf008_u16)]
    #[regex(r" +testset +", |_| 0xf00b_u16)]
    IRX(u16),

    // Jumps
    #[regex(r" +jump[a-zA-Z]?[a-zA-Z]? +(?:(?:[a-zA-Z][a-zA-z0-9_]*)|(?:\$[A-Fa-f0-9]{4}))(?:\[[Rr][(?:[0-9])(?:1[0-5])]])?", |lex| lex.slice().to_owned())]
    Jump(String),
}
