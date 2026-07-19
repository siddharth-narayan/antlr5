use std::{env, fs::read_to_string};

use crate::{codegen::symbols::SymbolTable, antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR, langs::{codegen, jinja_env}};

mod antlr;
mod codegen;
mod langs;

fn main() -> Result<(), ()> {
    let path = std::env::args().nth(1).unwrap();
    let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // println!("{}", content);

    // Lex + Parse
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    let ir = AntlrIR::new(ast);

    println!("{:#?}", ir);
    // let env = jinja_env(symbols);

    // std::fs::write("out", env.get_template("rust-parse").unwrap().render(ast).unwrap()).unwrap();
    
    Ok(())
}
