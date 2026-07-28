use std::{fs::read_to_string, hint::black_box, sync::Arc};

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR};

#[cfg(test)]
mod tests;

mod antlr;
mod codegen;
mod langs;

fn main() -> Result<(), ()> {
    tracing_subscriber::fmt().with_ansi(false).without_time().init();
    
    let path = std::env::args().nth(1).unwrap_or("src/tests/lookahead.g4".into());
    let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // println!("{}", content);

    // Lex + Parse
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    let mut ir = AntlrIR::new(ast);

    println!("{:#?}", ir.nth(ir.get_alt(0, 0).unwrap(), 0));

    Ok(())
}
