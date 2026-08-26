#![feature(titlecase)]
#![feature(option_into_flat_iter)]

use std::{collections::{HashSet, VecDeque}, fs::read_to_string, hint::black_box, sync::Arc};

use crate::{antlr::{lex::Lexer, parse::Parser}, codegen::{analysis::{HashSetMap, nth}, intermediate::AntlrIR}, langs::{Language, jinja_env, output}};

#[cfg(test)]
mod tests;

mod antlr;
mod codegen;
mod langs;
mod util;

fn main() -> Result<(), ()> {
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

    let x = 
        "grammar lookahead;
        n: x ;
        x: y z ;
        y: 'y' ;
        z: 'z' ;
        ";

    let lexer = Lexer::new(x.to_string());
    let mut parser = Parser::new(lexer).unwrap();

    let ast = parser.grammar_spec().unwrap();
    let ir = AntlrIR::new(ast);
    
    println!("{:#?}", ir.symbols());

    let alt = ir.get_rule_alt(0, 0).unwrap();

    let mut nth_cache = HashSetMap::new();
    let n1 = nth(1, 0, (alt.clone(), 0), &mut VecDeque::new(), &mut nth_cache, &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    println!("n1_alt0: {:#?}", n1);
    println!("NTH CACHCHCEHE{:#?}", nth_cache);


    // let n1_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
    //     ElementIR::RuleAtom { id: 1, suffix: None }, // x
    //     ElementIR::RuleAtom { id: 3, suffix: None }, // z

    //     ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    // ]);

    // assert_eq!(n1, n1_expected);

    Ok(())
}
