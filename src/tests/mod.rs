use std::fs::read_to_string;

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR};

pub fn parse(content: &'static str) -> AntlrIR {
    // Lex + Parse
    let lexer = Lexer::new(content.into());
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    
    AntlrIR::new(ast)
}

#[test]
pub fn cobol() {
    parse(include_str!("cobol.g4"));
}

#[test]
pub fn codegen() {
    parse(include_str!("codegen-test.g4"));
}

#[test]
pub fn parrt_test() {
    parse(include_str!("parrt-test.g4"));
}