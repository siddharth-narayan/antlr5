use std::sync::Arc;

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR, langs::jinja_env};

pub fn parse(content: &'static str) -> AntlrIR {
    // Lex + Parse
    let lexer = Lexer::new(content.into());
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    
    AntlrIR::new(ast)
}

pub fn codegen(ir: AntlrIR) {
    let ir = Arc::new(ir);
    let env = jinja_env(ir.clone());
    env.get_template("rust-parse").unwrap().render(ir.clone()).unwrap();
}

#[test]
pub fn cobol() {
    let ir = parse(include_str!("cobol.g4"));
    codegen(ir);
}

#[test]
pub fn lookahead() {
    let ir = parse(include_str!("lookahead.g4"));
    codegen(ir);
}

#[test]
pub fn parrt_test() {
    let ir = parse(include_str!("parrt-test.g4"));
    codegen(ir);
}