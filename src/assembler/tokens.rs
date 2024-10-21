use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Tokens {
    // Non-Instructions
    #[regex(r"[ \t\n\f]+")]
    Ignore,
    #[regex(r";.*[\r\n]")]
    Comment,
    #[regex(r"[a-zA-Z]+ +data +[(?:$[a-fA-F0-9]{1,4})(?:[0-9]+)]", |lex| lex.slice().to_owned())]
    Data(String),
    #[regex(r"[Rr][(?:[0-9])(?:1[0-5])],[Rr][(?:[0-9])(?:1[0-5])],[Rr][(?:[0-9])(?:1[0-5])]", |lex| lex.slice().to_owned())]
    RRR(String),
    #[regex(r"[Rr][(?:[0-9])(?:1[0-5])],[(?:[a-zA-Z][a-zA-Z0-9]+)(?:$[a-fA-F0-9]{1,4})]\[[Rr][(?:[0-9])(?:1[0-5])]]", |lex| lex.slice().to_owned())]
    IRX(String),
    
    // RRR Instructions
    #[token("add")]
    Add,
    #[token("sub")]
    Sub,
    #[token("mul")]
    Mul,
    #[token("div")]
    Div,
    #[token("cmp")]
    Cmp,
    #[token("addc")]
    Addc,
    #[token("muln")]
    Muln,
    #[token("divn")]
    Divn,
    #[token("rrr1")]
    Rrr1,
    #[token("rrr2")]
    Rrr2,
    #[token("rrr3")]
    Rrr3,
    #[token("rrr4")]
    Rrr4,
    #[token("trap")]
    Trap,
    
    // iRX Instructions
    #[token("lea")]
    Lea,
    #[token("load")]
    Load,
    #[token("store")]
    Store,
    #[token("jump")]
    Jump,
    #[token("jumpc0")]
    Jumpc0,
    #[token("jumpc1")]
    Jumpc1,
    #[token("jal")]
    Jal,
    #[token("jumpnz")]
    Jumpnz,
    #[token("jumpz")]
    Jumpz,
    #[token("testset")]
    TestSet,
}
