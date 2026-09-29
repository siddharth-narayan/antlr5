#![feature(titlecase)]
#![feature(option_into_flat_iter)]
#![allow(unused)] // Temporary
#![feature(path_absolute_method)]

use std::{collections::{HashSet, VecDeque}, fs::read_to_string, hash::RandomState, hint::black_box, sync::Arc, time::Instant};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

use crate::{antlr::lex::{ANTLRTokenType, Lexer}, codegen::{analysis::{match_rule, nth}, intermediate::{AntlrIR, element::ElementIR}}, langs::{Language, output}, util::HashSetMap};

#[cfg(test)]
mod tests;

mod antlr;
mod codegen;
mod langs;
mod util;

fn main() -> Result<(), ()> {
    let now = Instant::now();
    if std::env::args().any(|arg| arg == "--debug") {
        let subscriber = Registry::default().with(
            HierarchicalLayer::new(4)
                .with_indent_lines(true)
                .with_targets(true),
        );

        tracing::subscriber::set_global_default(subscriber).unwrap();
    }

    let path = std::env::args().nth(1).unwrap_or("src/antlr/antlr-updated.g4".into());
    let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // Lex + Parse
    let mut lexer = Lexer::new(content);
    
    

    // Unbootstrapped
    // let mut parser = parse::Parser::new(lexer).unwrap();

    // let ast = parser.grammar_spec().unwrap();
    // let ir = Arc::new(AntlrIR::new(ast));

    // output(ir.clone(), "out.rs".into(), Language::Rust);







    // Bootstrapped
    let mut tokens = Vec::new();
    while let Ok(token) = lexer.next_token() {
        if token.token_type() == ANTLRTokenType::EOF {
            break;
        }
        
        if token.token_type() == ANTLRTokenType::WS {
            continue;
        }

        tokens.push(token)
    }

    let mut parser = antlr::parser::Parser::new(tokens);
    let ast = parser.grammarSpec().unwrap();
    
    println!("{:#?}", ast);


    Ok(())
}