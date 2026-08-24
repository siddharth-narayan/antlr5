use std::sync::Arc;

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR, langs::jinja_env, tests::parse};

pub fn codegen(ir: AntlrIR) {
    let ir = Arc::new(ir);
    let env = jinja_env(ir.clone());
    env.get_template("rust-parse").unwrap().render(ir.clone()).unwrap();
}

#[test]
#[should_panic(expected = "No rule id found")]
pub fn antlr() {
    let lexer_ir = parse(include_str!("ANTLRv4Lexer.g4"));
    let parser_ir = parse(include_str!("ANTLRv4Parser.g4"));

    codegen(lexer_ir);
    codegen(parser_ir);
}