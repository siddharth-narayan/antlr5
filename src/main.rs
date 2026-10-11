#![feature(titlecase)]
#![feature(option_into_flat_iter)]
#![allow(unused)] // Temporary
#![feature(path_absolute_method)]

use std::{collections::{HashSet, VecDeque}, fs::read_to_string, hash::RandomState, hint::black_box, sync::Arc, time::Instant};

use clap::{Parser, command};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tracing::{error, level_filters::LevelFilter};
use tracing_subscriber::{Registry, fmt, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

use crate::{antlr::{lex::Lexer, parse}, codegen::{analysis::{match_rule, nth}, intermediate::{AntlrIR, element::ElementIR}}, langs::{Language, output}, util::HashSetMap};

#[cfg(test)]
mod tests;

mod antlr;
mod cli;
mod codegen;
mod langs;
mod util;

fn main() -> Result<(), ()> {
    let args = cli::Args::parse();
    let subscriber = fmt::fmt()
        .with_file(true)
        .without_time()
        .with_target(false)
        .with_line_number(true)
        .with_max_level(args.log_level).init();

    let content = read_to_string(&args.path).map_err(|e| { error!("Failed to read file {} {}", args.path.into_string().unwrap_or("".into()), e); })?;

    // Lex + Parse
    let lexer = Lexer::new(content);
    
    

    // Unbootstrapped
    let mut parser = parse::Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    let ir = Arc::new(AntlrIR::new(ast));

    output(ir.clone(), "out.rs".into(), Language::Rust);







    // Bootstrapped
    // let mut tokens = Vec::new();
    // while let Ok(token) = lexer.next_token() {
    //     tokens.push(token)
    // }

    // let mut parser = Parser::new(tokens);
    // let ast = parser.gSpec().unwrap();
    
    // println!("{:#?}", ast);


    Ok(())
}