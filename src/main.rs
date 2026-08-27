#![feature(titlecase)]
#![feature(option_into_flat_iter)]

use std::{collections::{HashSet, VecDeque}, fs::read_to_string, hash::RandomState, hint::black_box, sync::Arc};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::{analysis::{HashSetMap, nth}, intermediate::{AntlrIR, element::ElementIR}}, langs::{Language, jinja_env, output}};

#[cfg(test)]
mod tests;

mod antlr;
mod codegen;
mod langs;
mod util;

fn main() -> Result<(), ()> {
    if std::env::args().any(|arg| arg == "--debug") {
        let subscriber = Registry::default().with(
            HierarchicalLayer::new(4)
                .with_indent_lines(true)
                .with_targets(true),
        );

        tracing::subscriber::set_global_default(subscriber).unwrap();
    }

    let path = std::env::args().nth(1).unwrap_or("src/tests/cobol.g4".into());
    let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // Lex + Parse
    let lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    let ir = Arc::new(AntlrIR::new(ast));
    
    (0..ir.rules().len()).into_par_iter().for_each(
        |rule_id| {
            let mut cache = HashSetMap::new();
            println!("Calculating nth sets for rule {}", rule_id);
            for n in 0..=25 {
                let mut set = HashSet::new();

                for alt in ir.get_rule(rule_id).unwrap().alts() {
                    let n = crate::nth(n, 0, (alt.clone(), 0), &mut VecDeque::new(), &mut cache, &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
                    set.extend(n);
                }

                // println!("NTH set for n = {}: {:#?}", n, set);
                black_box(set);
            }
        }
    );

    // // let jinja_env = jinja_env(ir.clone());

    // // output(ir.clone(), "out", jinja_env, Language::Rust);
    
    // black_box(ir);

    Ok(())
}