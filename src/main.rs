#![feature(titlecase)]
#![feature(option_into_flat_iter)]

use std::{collections::{HashSet, VecDeque}, fs::read_to_string, hash::RandomState, hint::black_box, sync::Arc};

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
    let subscriber = Registry::default().with(
        HierarchicalLayer::new(4)
            .with_indent_lines(true)
            .with_targets(true),
    );

    tracing::subscriber::set_global_default(subscriber).unwrap();

    // let path = std::env::args().nth(1).unwrap_or("src/tests/lookahead.g4".into());
    // let content = read_to_string(path).map_err(|e| { println!("{}", e); })?;

    // // Lex + Parse
    // let lexer = Lexer::new(content);
    // let mut parser = Parser::new(lexer).unwrap();

    // let ast = parser.grammar_spec().unwrap();
    // let ir = AntlrIR::new(ast);
    
    // let mut cache = HashSetMap::new();

    // nth(0, 0, (ir.get_rule_alt(0, 0).unwrap(), 0), &mut VecDeque::new(), &mut cache, &mut HashSet::new(), ir.rules());
    // println!("{:#?}", cache.get(&(ir.get_rule_alt(0, 0).unwrap(), 0)));



    // let ir = Arc::new(ir);
    // // let jinja_env = jinja_env(ir.clone());

    // // output(ir.clone(), "out", jinja_env, Language::Rust);
    
    // black_box(ir);

    // Ok(())

    // let n1_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
    //     ElementIR::RuleAtom { id: 1, suffix: None }, // x
    //     ElementIR::RuleAtom { id: 3, suffix: None }, // z

    //     ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    // ]);

    // assert_eq!(n1, n1_expected);

    n();
    Ok(())
}


static GRAMMAR: &'static str = 
    "grammar lookahead ;
    n: x y | y x ;
    x: y z ;
    y: 'y' ;
    z: 'z' ;

    optional: y? z ;
    star: y* z ;
    ";

    pub fn parse(content: &'static str) -> AntlrIR {
    // Lex + Parse
    let lexer = Lexer::new(content.into());
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    
    AntlrIR::new(ast)
}

pub fn n() {
    let ir = parse(GRAMMAR);

    let n0_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::RuleAtom { id: 1, suffix: None },
        ElementIR::RuleAtom { id: 2, suffix: None },

        ElementIR::TokenAtom { id: 0, suffix: None },
    ]);
    
    let n1_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::RuleAtom { id: 1, suffix: None }, // x
        ElementIR::RuleAtom { id: 2, suffix: None }, // y
        ElementIR::RuleAtom { id: 3, suffix: None }, // z

        ElementIR::TokenAtom { id: 0, suffix: None }, // 'y'
        ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    ]);

    let n2_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::RuleAtom { id: 1, suffix: None }, // x
        ElementIR::RuleAtom { id: 2, suffix: None }, // y
        ElementIR::RuleAtom { id: 3, suffix: None }, // z

        ElementIR::TokenAtom { id: 0, suffix: None }, // 'y'
        ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    ]);

    assert_eq!(ir.nth(0, 0).unwrap(), n0_expected);
    assert_eq!(ir.nth(1, 0).unwrap(), n1_expected);
    // THIS SHOULD WORKKKKK
    assert_eq!(ir.nth(2, 0).unwrap(), n2_expected);
}