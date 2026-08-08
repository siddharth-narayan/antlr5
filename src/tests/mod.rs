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
#[should_panic(expected = "No rule id found")]
pub fn antlr() {
    let lexer_ir = parse(include_str!("ANTLRv4Lexer.g4"));
    let parser_ir = parse(include_str!("ANTLRv4Parser.g4"));

    codegen(lexer_ir);
    codegen(parser_ir);
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
    let ir = parse(include_str!("parrt_test.g4"));
    codegen(ir);
}

// #[test]
// pub fn always_contains() {
//     let ir = parse(include_str!("recursion.g4"));

//     let x = ir.symbols().get_rule_id(&"x".to_string()).unwrap();
//     let y = ir.symbols().get_rule_id(&"y".to_string()).unwrap();

//     assert!(ir.rule_always_contains(y, x))
// }

// #[test]
// pub fn not_always_contains() {
//     let ir = parse(include_str!("recursion.g4"));

//     let x = ir.symbols().get_rule_id(&"x".to_string()).unwrap();
//     let z = ir.symbols().get_rule_id(&"z".to_string()).unwrap();

//     assert!(!ir.rule_always_contains(z, x))
// }