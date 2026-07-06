use std::{env, fs::read_to_string};

use crate::{antlr::{Lexer, Parser}, langs::{codegen, jinja_env}};

mod ast;
mod antlr;
mod langs;
mod codegen;

fn main() -> Result<(), ()> {
    let path = "test.g4";
    let content = read_to_string(path).map_err(|e| {println!("{}", e); ()})?;

    // println!("{}", content);

    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let result = parser.grammar_spec().unwrap();
    // println!("{:#?}", result);

    let env = jinja_env();
    codegen(env, result);
    Ok(())
}
