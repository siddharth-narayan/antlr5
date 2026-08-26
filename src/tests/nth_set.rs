use std::{collections::HashSet, hash::RandomState};

use crate::{codegen::{analysis::{HashSetMap, nth}, intermediate::element::ElementIR}, tests::parse};

#[test]
pub fn nth_set() {
    let x = 
        "grammar lookahead;
        n: x y | y x ;
        x: y z ;
        y: 'y' ;
        z: 'z' ;
        ";

    let ir = parse(x);

    println!("{:#?}", ir.symbols());

    let alt0 = ir.get_rule_alt(0, 0).unwrap();
    let alt1 = ir.get_rule_alt(0, 1).unwrap();

    let n0_alt0 = nth(0, 0, (alt0.clone(), 0), None, &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    let n0_alt1 = nth(0, 0, (alt1.clone(), 0), None, &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    let n0: HashSet<ElementIR, RandomState> = n0_alt0.union(&n0_alt1).cloned().collect();

    let n1_alt0 = nth(1, 0, (alt0.clone(), 0), None, &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    let n1_alt1 = nth(1, 0, (alt1.clone(), 0), None, &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    let n1: HashSet<ElementIR, RandomState> = n1_alt0.union(&n1_alt1).cloned().collect();


    let n2_alt0 = nth(2, 0, (alt0.clone(), 0), None, &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
    println!("n2_alt0: {:#?}", n2_alt0);
    let n2_alt1 = nth(2, 0, (alt1.clone(), 0), None, &mut HashSetMap::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default();
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
