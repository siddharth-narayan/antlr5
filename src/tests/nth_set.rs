use std::{collections::{HashSet, VecDeque}, hash::RandomState};

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{HashSetMap, nth}, intermediate::element::ElementIR}, tests::parse};


static GRAMMAR: &'static str = 
    "grammar lookahead ;
    n: x y | y x ;
    x: y z ;
    y: 'y' ;
    z: 'z' ;

    optional: y? z ;
    star: y* z ;
    ";

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

#[test]
pub fn x() {
    let ir = parse(GRAMMAR);

    let x0_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::RuleAtom { id: 2, suffix: None }, // y
        ElementIR::TokenAtom { id: 0, suffix: None }, // 'y'
    ]);

    let x1_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::RuleAtom { id: 3, suffix: None }, // z
        ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    ]);

    let x2_expected: HashSet<ElementIR, RandomState> = HashSet::new();

    assert_eq!(ir.nth(0, 1).unwrap(), x0_expected);
    assert_eq!(ir.nth(1, 1).unwrap(), x1_expected);
    assert_eq!(ir.nth(2, 1).unwrap(), x2_expected);
}

#[test]
pub fn optional() {
    let ir = parse(GRAMMAR);

    let expected_0: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::RuleAtom { id: 2, suffix: Some(EBNFSuffix::Optional) }, // y
        ElementIR::TokenAtom { id: 0, suffix: None }, // 'y'
        ElementIR::RuleAtom { id: 3, suffix: None }, // z
        ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    ]);

    let expected_1: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::RuleAtom { id: 3, suffix: None }, // z
        ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    ]);

    let expected_2: HashSet<ElementIR, RandomState> = HashSet::new();

    assert_eq!(ir.nth(0, 4).unwrap(), expected_0);
    assert_eq!(ir.nth(1, 4).unwrap(), expected_1);
    assert_eq!(ir.nth(2, 4).unwrap(), expected_2);
}

#[test]
pub fn star() {
    let ir = parse(GRAMMAR);

    let expected_0: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::RuleAtom { id: 2, suffix: Some(EBNFSuffix::Star) }, // y
        ElementIR::TokenAtom { id: 0, suffix: None }, // 'y'
        ElementIR::RuleAtom { id: 3, suffix: None }, // z
        ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    ]);

    let expected_1: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::RuleAtom { id: 3, suffix: None }, // z
        ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    ]);

    let expected_2: HashSet<ElementIR, RandomState> = HashSet::new();

    assert_eq!(ir.nth(0, 5).unwrap(), expected_0);
    assert_eq!(ir.nth(1, 5).unwrap(), expected_1);
    assert_eq!(ir.nth(2, 5).unwrap(), expected_2);
}

#[test]
pub fn offset_check() {
    let ir = parse(GRAMMAR);

    println!("{:#?}", ir.symbols());

    let alt = ir.get_rule_alt(1, 0).unwrap();

    let mut nth_cache = HashSetMap::new();
    let n1 = nth(0, 0, (alt.clone(), 0), &mut VecDeque::new(), &mut nth_cache, &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    // println!("n1_alt0: {:#?}", n1);
    // println!("NTH CACHCHCEHE{:#?}", nth_cache);


    // let n1_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
    //     ElementIR::RuleAtom { id: 1, suffix: None }, // x
    //     ElementIR::RuleAtom { id: 3, suffix: None }, // z

    //     ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    // ]);

    // assert_eq!(n1, n1_expected);


}
