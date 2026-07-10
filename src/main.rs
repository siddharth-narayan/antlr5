use std::{env, fs::read_to_string};

use crate::{analysis::SymbolTable, antlr::{lex::Lexer, parse::Parser}, langs::{codegen, jinja_env}};

mod ast;
mod antlr;
mod codegen;
mod analysis;
mod langs;

fn main() -> Result<(), ()> {
    let path = std::env::args().nth(1).unwrap();
    let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // println!("{}", content);

    // Lex + Parse
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    println!("{:#?}", ast);

    // Analysis
    let mut symbols = SymbolTable::new();
    ast.symbols(&mut symbols).unwrap();
    println!("{:#?}", symbols);

    // Codegen
    let atn = ast.codegen(&mut symbols);

    let env = jinja_env();
    
    std::fs::write("parser.gen", env.get_template("rust-parse").unwrap().render(ast).unwrap())
    ;
    println!("{:#?}", atn);
    
    Ok(())
}
