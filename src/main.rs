use std::{env, fs::read_to_string};

use crate::{analysis::SymbolTable, antlr::{Lexer, Parser}, langs::{codegen, jinja_env}};

mod ast;
mod antlr;
mod codegen;
mod analysis;
mod langs;

fn main() -> Result<(), ()> {
    let path = "test.g4";
    let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // println!("{}", content);

    // Lex + Parse
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let result = parser.grammar_spec().unwrap();
    println!("{:#?}", result);

    // Analysis
    let mut symbols = SymbolTable::new();
    result.symbols(&mut symbols).unwrap();
    println!("{:#?}", symbols);

    // Codegen
    let atn = result.codegen(&mut symbols);

    let env = jinja_env();
    // println!("{:#?}", atn);
    Ok(())
}
