use std::{collections::HashSet, hash::RandomState};

use crate::{codegen::{analysis::{Cache, nth}, intermediate::element::ElementIR}, tests::parse};

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

    let alt0 = ir.get_rule_alt(0, 0).unwrap();
    let alt1 = ir.get_rule_alt(0, 1).unwrap();

    let mut n0 = nth(0, 0, (alt0.clone(), 0), None, &mut Cache::new(), &mut HashSet::new(), ir.rules()).unwrap().clone();
    n0.extend(nth(0, 0, (alt1.clone(), 0), None, &mut Cache::new(), &mut HashSet::new(), ir.rules()).unwrap().clone());
    
    let mut n1 = nth(1, 0, (alt0.clone(), 0), None, &mut Cache::new(), &mut HashSet::new(), ir.rules()).unwrap().clone();
    n1.extend(nth(0, 0, (alt1.clone(), 0), None, &mut Cache::new(), &mut HashSet::new(), ir.rules()).unwrap().clone());

    let mut n2 = nth(2, 0, (alt0, 0), None, &mut Cache::new(), &mut HashSet::new(), ir.rules()).cloned().unwrap_or_default().clone();
    n2.extend(nth(0, 0, (alt1, 0), None, &mut Cache::new(), &mut HashSet::new(), ir.rules()).unwrap().clone());


    let n0_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::TokenAtom { id: 0, suffix: None },
        ElementIR::RuleAtom { id: 1, suffix: None },
        ElementIR::RuleAtom { id: 2, suffix: None }
    ]);
    
    let n1_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::TokenAtom { id: 0, suffix: None },
        ElementIR::RuleAtom { id: 2, suffix: None }
    ]);

    let n2_expected: HashSet<ElementIR, RandomState> = HashSet::from_iter(vec![
        ElementIR::TokenAtom { id: 0, suffix: None },
        ElementIR::RuleAtom { id: 2, suffix: None }
    ]);

    assert_eq!(n0, n0_expected);
    assert_eq!(n1, n1_expected);
    assert_eq!(n2, n2_expected);

}
