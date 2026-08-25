use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR};

mod codegen;
mod nth_set;

pub fn parse(content: &'static str) -> AntlrIR {
    // Lex + Parse
    let lexer = Lexer::new(content.into());
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    
    AntlrIR::new(ast)
}