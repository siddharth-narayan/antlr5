#![feature(titlecase)]
#![feature(option_into_flat_iter)]

use std::{collections::HashSet, fs::read_to_string, hint::black_box, sync::Arc};

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::{analysis::{Cache, nth}, intermediate::AntlrIR}, langs::{Language, jinja_env, output}};

#[cfg(test)]
mod tests;

mod antlr;
mod codegen;
mod langs;
mod util;

fn main() -> Result<(), ()> {
    let path = std::env::args().nth(1).unwrap_or("src/tests/lookahead.g4".into());
    let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // Lex + Parse
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    let ir = AntlrIR::new(ast);
    
    let mut cache = Cache::new();

    nth(0, 0, (ir.get_rule_alt(0, 0).unwrap(), 0), None, &mut cache, &mut HashSet::new(), ir.rules());
    println!("{:#?}", cache.get(&(ir.get_rule_alt(0, 0).unwrap(), 0)));



    let ir = Arc::new(ir);
    // let jinja_env = jinja_env(ir.clone());

    // output(ir.clone(), "out", jinja_env, Language::Rust);
    
    black_box(ir);

    Ok(())
}
