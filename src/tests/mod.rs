use std::sync::Arc;

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR};

mod codegen;
mod nth_set;
mod matches;

pub fn parse(content: &'static str) -> AntlrIR {
    // Lex + Parse
    let lexer = Lexer::new(content.into());
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    
    AntlrIR::new(ast)
}

pub fn get_id_closures(ir: Arc<AntlrIR>) -> (impl Fn(&'static str) -> usize, impl Fn(&'static str) -> usize,) {
    let ir_clone = ir.clone();
    let token_id = move |name: &'static str| {
        println!("Searching for a token named \"{}\"", name);
        ir_clone.symbols().get_token_id(&String::from(name)).unwrap()
    };

    let strlit_id = move |name: &'static str| {
        println!("Searching for a strlit named \"{}\"", name);
        ir.clone().symbols().get_strlit_id(&String::from(name)).unwrap()
    };

    (token_id, strlit_id)
}