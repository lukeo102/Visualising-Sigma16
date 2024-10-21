use logos::Logos;
use crate::assembler::tokens::Tokens;

pub fn parse_code(code: &str) {
    let mut lexer = Tokens::lexer(code);
    
    for token in lexer {
        println!("{:?}", token);
    }
}
