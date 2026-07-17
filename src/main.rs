use std::{env, fs::read_to_string};

use crate::{analysis::SymbolTable, antlr::{lex::Lexer, parse::Parser}, langs::{codegen, jinja_env}};

mod ast;
mod antlr;
mod analysis;
mod langs;
mod intermediate;

fn main() -> Result<(), ()> {
    let path = std::env::args().nth(1).unwrap();
    let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // println!("{}", content);

    // Lex + Parse
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    println!("{:#?}", ast);

    // let env = jinja_env(symbols);

    // std::fs::write("out", env.get_template("rust-parse").unwrap().render(ast).unwrap()).unwrap();
    
    Ok(())
}
