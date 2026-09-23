
use std::{sync::Arc, time::{Duration, Instant}};

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::intermediate::AntlrIR, langs::{Language, output, render}, tests::parse};

#[test]
pub fn codegen() {
    let ir = parse(include_str!("grammars/cobol.g4"));
    
    let time = Instant::now();
    render(ir.into(), Language::Rust, "/dev/null");
    
    let elapsed  = time.elapsed();
    println!("Elapsed: {}ms", elapsed.as_millis());
    assert!(elapsed < Duration::from_millis(500)) 
}