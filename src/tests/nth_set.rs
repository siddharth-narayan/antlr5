use std::{collections::{HashSet, VecDeque}, hash::RandomState};

use crate::{codegen::{analysis::{HashSetMap, nth}, intermediate::element::ElementIR}, tests::parse};

#[test]
pub fn nth_set() {
    let x = 
        "grammar lookahead;
        n: x y 
            | y x ; // Unintuively, x will not be included in the nth(2) set, because it doesn't start with the nth nonterminal (z)
        x: y z ;
        y: 'y' ;
        z: 'z' ;
        ";

    let ir = parse(x);

    println!("{:#?}", ir.symbols());

    let alt0 = ir.get_rule_alt(0, 0).unwrap();
    let alt1 = ir.get_rule_alt(0, 1).unwrap();

    let n0_alt0 = nth(0, 0, (alt0.clone(), 0), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    let n0_alt1 = nth(0, 0, (alt1.clone(), 0), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    let n0: HashSet<ElementIR, RandomState> = n0_alt0.union(&n0_alt1).cloned().collect();

    let n1_alt0 = nth(1, 0, (alt0.clone(), 0), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    let n1_alt1 = nth(1, 0, (alt1.clone(), 0), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    let n1: HashSet<ElementIR, RandomState> = n1_alt0.union(&n1_alt1).cloned().collect();


    let n2_alt0 = nth(2, 0, (alt0.clone(), 0), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    println!("n2_alt0: {:#?}", n2_alt0);
    let n2_alt1 = nth(2, 0, (alt1.clone(), 0), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    println!("n2_alt1: {:#?}", n2_alt1);
    let n2: HashSet<ElementIR, RandomState> = n2_alt0.union(&n2_alt1).cloned().collect();

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

    assert_eq!(n0, n0_expected);
    assert_eq!(n1, n1_expected);
    assert_eq!(n2, n2_expected);

}


#[test]
pub fn offset_check() {
    let x = 
        "grammar lookahead;
        n: x ;
        x: y z ;
        y: 'y' ;
        z: 'z' ;
        ";

    let ir = parse(x);

    println!("{:#?}", ir.symbols());

    let alt = ir.get_rule_alt(1, 0).unwrap();

    let mut nth_cache = HashSetMap::new();
    let n1 = nth(0, 0, (alt.clone(), 0), &mut VecDeque::new(), &mut nth_cache, &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    // println!("n1_alt0: {:#?}", n1);
    println!("NTH CACHCHCEHE{:#?}", nth_cache);


    // let n1_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
    //     ElementIR::RuleAtom { id: 1, suffix: None }, // x
    //     ElementIR::RuleAtom { id: 3, suffix: None }, // z

    //     ElementIR::TokenAtom { id: 1, suffix: None }, // 'z'
    // ]);

    // assert_eq!(n1, n1_expected);


}
