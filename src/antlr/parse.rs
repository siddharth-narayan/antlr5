use crate::antlr::{ANTLRToken, Lexer};

#[derive(Debug)]
pub enum ParserErr {
    
}

pub struct Parser {
    tokens: Vec<ANTLRToken>
}

impl Parser {
    pub fn new(lexer: Lexer) {

    }
}