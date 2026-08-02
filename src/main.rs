#![feature(titlecase)]

use std::{fs::read_to_string, hint::black_box, sync::Arc};

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR, langs::{Language, jinja_env, output}};

#[cfg(test)]
mod tests;

mod antlr;
mod codegen;
mod langs;

fn main() -> Result<(), ()> {
    tracing_subscriber::fmt().with_ansi(false).without_time().init();
    
    let path = std::env::args().nth(1).unwrap_or("src/tests/cobol.g4".into());
    let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // Lex + Parse
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    let ir = AntlrIR::new(ast);
    // println!("{:#?}", ir.symbols());

    // for index in 0..ir.rules().len() {
        // println!("Calculating lookahead for rule {} ({})", ir.symbols().get_rule_name(index).unwrap(), index);
        // let la = stacker::grow(4 * 1024 * 1024 * 1024, || ir.lookahead(index));
        // println!("Lookahead {}: {:#?}", index, la);
        // black_box(la);
    // }

    // let la = ir.lookahead(327);
    // let la = ir.nth(ir.get_alt(327, 0).unwrap().clone(), 65);
    // println!("Output: {:#?}", la);
    // black_box(la);


    let ir = Arc::new(ir);
    let jinja_env = jinja_env(ir.clone());

    output(ir.clone(), "out", jinja_env, Language::Rust);
    
    black_box(ir);

    Ok(())
}
