#![feature(titlecase)]
#![feature(option_into_flat_iter)]
#![allow(unused)] // Temporary
#![feature(path_absolute_method)]

use std::{collections::{HashSet, VecDeque}, fs::read_to_string, hash::RandomState, hint::black_box, sync::Arc, time::Instant};

use clap::{Parser, command};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tracing::{error, instrument::WithSubscriber, level_filters::LevelFilter};
use tracing_subscriber::{Layer, Registry, fmt, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

use crate::{antlr::{lex::Lexer, parse}, codegen::{analysis::{algorithm::nth, r#match::match_rule}, intermediate::{AntlrIR, element::ElementIR}}, langs::{Language, output}, util::HashSetMap};

#[cfg(test)]
mod tests;

mod antlr;
mod cli;
mod codegen;
mod langs;
mod util;

fn main() -> Result<(), ()> {
    let args = cli::Args::parse();

    let subscriber = Registry::default()
        .with(
            HierarchicalLayer::new(4)
                .with_indent_lines(true)
                .with_targets(true)
                .with_filter(LevelFilter::from_level(args.log_level))
        );

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let content = read_to_string(&args.path).map_err(|e| { error!("Failed to read file {} {}", args.path.into_string().unwrap_or("".into()), e); })?;

    // Lex + Parse
    let lexer = Lexer::new(content);
    let mut parser = parse::Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    let ir = Arc::new(AntlrIR::new(ast));

    // println!("{:#?}", match_rule(ir.clone(), 1));
    output(ir.clone(), "out.rs".into(), Language::Rust);

    Ok(())
}