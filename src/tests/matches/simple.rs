use std::sync::Arc;

use crate::{codegen::{analysis::{MatchNode, match_element}, intermediate::element::ElementIR}, tests::{get_id_closures, parse}};

static GRAMMAR: &'static str = 
"grammar matches;
s: z s x | LETTERB 'b' LETTERA 'a' | LETTERA 'a' LETTERB 'b';
z: LETTERC 'c' ;
x: LETTERN 'n' ;
LETTERA: 'a' ;
LETTERB: 'b' ;
LETTERN: 'n' ;
LETTERC: 'c' ;";

#[test]
pub fn match_alt0() {
    let ir = Arc::new(parse(GRAMMAR));

    let alt0 = ir.get_rule_alt(0, 0).unwrap();

    let alt0_expected = MatchNode::Element { 
        alt: alt0.clone(),
        element_idx: 0,
        element: ElementIR::RuleAtom { id: 1, suffix: None },
        next: Box::new(
            MatchNode::Element {
                alt: alt0.clone(),
                element_idx: 1,
                element: ElementIR::RuleAtom { id: 0, suffix: None },
                next: Box::new(
                    MatchNode::Element {
                        alt: alt0.clone(),
                        element_idx: 2,
                        element: ElementIR::RuleAtom { id: 2, suffix: None },
                        next: Box::new(MatchNode::Finish { alt: alt0.clone() })
                    }
                )
            }
        )
    };

   

    println!("{:#?}", ir.symbols());

    assert_eq!(match_element(alt0, 0, ir), alt0_expected);
}

#[test]
pub fn match_alt1() {
    let ir = Arc::new(parse(GRAMMAR));

    let alt1 = ir.get_rule_alt(0, 1).unwrap();

    let (token_id, strlit_id) = get_id_closures(ir.clone());

    let alt1_expected = MatchNode::Element { 
        alt: alt1.clone(),
        element_idx: 0,
        element: ElementIR::TokenAtom { id: token_id("LETTERB"), suffix: None },
        next: Box::new(
            MatchNode::Element {
                alt: alt1.clone(),
                element_idx: 1,
                element: ElementIR::TokenAtom { id: strlit_id("b"), suffix: None },
                next: Box::new(
                    MatchNode::Element {
                        alt: alt1.clone(),
                        element_idx: 2,
                        element: ElementIR::TokenAtom { id: token_id("LETTERA"), suffix: None },
                        next: Box::new(
                            MatchNode::Element {
                                alt: alt1.clone(),
                                element_idx: 3,
                                element: ElementIR::TokenAtom { id: strlit_id("a"), suffix: None },
                                next: Box::new(MatchNode::Finish { alt: alt1.clone() })
                            }
                        )
                    }
                )
            }
        )
    };

    assert_eq!(match_element(alt1, 0, ir), alt1_expected);
}