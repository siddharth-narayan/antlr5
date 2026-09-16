use std::sync::Arc;

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR, langs::{Language, output}, tests::parse};

pub fn codegen(ir: AntlrIR) {
    let ir = Arc::new(ir);
    output(ir.clone(),"/dev/null", Language::Rust);
}

#[test]
#[should_panic(expected = "No rule found")]
pub fn antlr() {
    let lexer_ir = parse(include_str!("ANTLRv4Lexer.g4"));
    let parser_ir = parse(include_str!("ANTLRv4Parser.g4"));

    codegen(lexer_ir);
    codegen(parser_ir);
}