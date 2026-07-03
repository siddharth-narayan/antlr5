use std::{env, fs::read_to_string};

use crate::antlr::{Lexer, Parser};

mod antlr;


fn main() -> Result<(), ()> {
    let path = "test.g4";
    let content = read_to_string(path).map_err(|e| {println!("{}", e); ()})?;

    // println!("{}", content);

    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let result = parser.grammar_spec().unwrap();
    println!("{:#?}", result);

    Ok(())
}
