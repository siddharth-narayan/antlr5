use std::{fs::read_to_string, hint::black_box, sync::Arc};

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR};

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
    let mut ir = AntlrIR::new(ast);
    println!("{:#?}", ir.symbols());

    for index in 0..ir.rules().len() {
        let la = ir.lookahead(index);
        println!("FIRST for rule {}: {:#?}", index, ir.rule_nth(index, 0));
        println!("Lookahead: {:#?}", la);
    }

    Ok(())
}
