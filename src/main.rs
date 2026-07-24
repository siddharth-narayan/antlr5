use std::{fs::read_to_string, hint::black_box, sync::Arc};

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR};

#[cfg(test)]
mod tests;

mod antlr;
mod codegen;
mod langs;

fn main() -> Result<(), ()> {
    let path = std::env::args().nth(1).unwrap_or("src/tests/parrt-test.g4".into());
    let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // println!("{}", content);

    // Lex + Parse
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    let ir = Arc::new(AntlrIR::new(ast));

    let first = ir.nth(ir.rules().first().unwrap().alts().get(0).unwrap(), 0);
    let second = ir.nth(ir.rules().get(1).unwrap().alts().get(0).unwrap(), 0);
    
    let la = ir.lookahead(0);
    println!("Lookahead for rule 0 is {:#?}", la);

    // let env = jinja_env(ir.clone());
    // std::fs::write("out", env.get_template("rust-parse").unwrap().render(ir.clone()).unwrap()).unwrap();
    
    black_box(first);
    black_box(second);
    black_box(ir.clone());

    Ok(())
}
