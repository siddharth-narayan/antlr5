use rapidhash::fast::RandomState;
use tracing::{error, instrument};
use std::{
    char, collections::{HashMap, HashSet, VecDeque, hash_set::IntoIter}, hash::Hash, mem::discriminant, sync::Arc,
};
use std::fmt::Debug;
use crate::{
    antlr::ast::{EBNFSuffix, Element}, codegen::{
        RuleRef, analysis::{algorithm::nth, numbers::{ConcreteOrdinal, Ordinal}, peek::{Peek, PeekResult}}, intermediate::{
            AntlrIR, alt::{self, AltIR}, element::{self, ElementIR}, rule::RuleIR,
        },
    }, util::{HashArc, HashSetMap},
};


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MatchNode {
    Peek(Peek),

    Element {
        alt: HashArc<AltIR>,
        element_idx: usize,

        element: ElementIR,
        next: Box<MatchNode>
    },

    Finish {
        alt: HashArc<AltIR>
    },
}

impl MatchNode {

// None - None       parsing - match exactly one   --- generation, skip to next element
// None - Option     parsing, if next token is correct, lookahead on next element, otherwise Option
// None - Star       
// None - Plus

// Option - Option // Exact
// Option - Star
// Option - Plus

// Star - Star
// Star - Plus

// Plus - Plus // Exact


// for exact matches we continue to the next element directly
    #[instrument(skip(ir))]
    pub fn merge(&mut self, mut other: MatchNode, ir: Arc<AntlrIR>) {
        match self {
            MatchNode::Peek { .. } => {
                match other {
                    MatchNode::Peek { .. } => self.merge_peek_peek(other, ir),

                    MatchNode::Element { .. } => self.merge_peek_element(other, ir),
                    
                    MatchNode::Finish { .. } => (),
                }
            },

            MatchNode::Element { .. } => {
                match other {
                    // Swap and recurse
                    MatchNode::Peek { .. } => {
                        std::mem::swap(self, &mut other);
                        self.merge(other, ir);
                    },
                    MatchNode::Element { .. } => self.merge_element_element(other, ir),
                    MatchNode::Finish { .. } => (),
                }
            }
            
            MatchNode::Finish { .. } => *self = other
        }
    }

    pub fn merge_peek_peek(&mut self, other: MatchNode, ir: Arc<AntlrIR>) {
        let MatchNode::Peek(peek_left) = self else { panic!() };
        let MatchNode::Peek(peek_right) = other else { panic!() };

        for (token_right, node_right) in peek_right.map {
                match peek_left.map.get_mut(&token_right) {
                    Some(match_left) => {
                        match_left.merge(node_right, ir.clone());
                    },
                    None => {
                        peek_left.map.insert(token_right, node_right);
                    }
                }
            }

            // Match fallback
            match &mut peek_left.fallback {
                Some(fallback_left) => {
                    if let Some(fallback_right) = peek_right.fallback {
                        fallback_left.merge(*fallback_right, ir);
                    };
                },
                None => {
                    peek_left.fallback = peek_right.fallback
                }
            };
    }

    pub fn merge_peek_element(&mut self, other: MatchNode, ir: Arc<AntlrIR>) {
        let MatchNode::Peek(peek_left) = self else { panic!() };
        let MatchNode::Element { alt, element_idx, element, next } = &other else { panic!() };
        
        match peek_left.map.get_mut(&element.id().unwrap_or_else(|| { error!("{:#?}", element); panic!() })) {
            Some(match_left) => {
                match_left.match_node.merge(other, ir);
            },
            None => {
                // Do we insert other here?
                peek_left.map.insert(element.id().unwrap(), PeekResult::new(other.clone(), ConcreteOrdinal::Finite(0), element.size()));
            }
        }
    }

    pub fn merge_element_element(&mut self, other: MatchNode, ir: Arc<AntlrIR>) {
        let MatchNode::Element { alt: alt_left, element_idx: element_idx_left, element: element_left, next: next_left } = &other else { panic!() };
        let MatchNode::Element { alt: alt_right, element_idx: element_idx_right, element: element_right, next: next_right } = &other else { panic!() };
        
        let mut peek = Peek::new();
        
        // NEED TO FIX THIS!!!
        // This is absolutely not how merging elements should work
        if let Some(nthset) = nth(0, 0, (alt_left.clone(), *element_idx_left), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::default(), ir.rules()) {
            for element in nthset {
                if let ElementIR::TokenAtom { .. } = element {
                    peek.map.insert(element.id().unwrap(), PeekResult::new(match_element_dumb(alt_left.clone(), *element_idx_left, ir.clone()), ConcreteOrdinal::Finite(0), Ordinal { is_optional: false, inner: ConcreteOrdinal::Infinite { offset: 0, multiple: 1 } } ));
                }
            }
        }
        
        if let Some(nthset) = nth(0, 0, (alt_right.clone(), *element_idx_right), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::default(), ir.rules()) {
            for element in nthset {
                if let ElementIR::TokenAtom { .. } = element {
                    peek.map.insert(element.id().unwrap(), PeekResult::new(match_element_dumb(alt_right.clone(), *element_idx_right, ir.clone()), ConcreteOrdinal::Finite(0), Ordinal { is_optional: false, inner: ConcreteOrdinal::Infinite { offset: 0, multiple: 1 } }));
                }
            }
        }
        
        *self = MatchNode::Peek(Peek::new());
    }
}

// Rule matches
pub fn match_element_dumb(alt: HashArc<AltIR>, element_idx: usize, ir: Arc<AntlrIR>) -> MatchNode {
    let element: ElementIR = alt.elements().get(element_idx).unwrap().clone();
    MatchNode::Element { alt: alt.clone(), element_idx, element: element.clone(), next: Box::new(match_element(alt.clone(), element_idx + 1, ir.clone())) }
}

#[instrument(skip(ir))]
pub fn match_element(alt: HashArc<AltIR>, element_idx: usize, ir: Arc<AntlrIR>) -> MatchNode {
    let element: ElementIR = match alt.elements().get(element_idx) {
        Some(e) => e.clone(),
        None => return MatchNode::Finish { alt }
    };

    let mut base = match_element_dumb(alt.clone(), element_idx, ir.clone());

    if element.size().is_optional {
        let next_element_match = match_element(alt, element_idx + 1, ir.clone());
        base.merge(next_element_match, ir);
    }
    
    return base
}

#[instrument(skip(ir))]
pub fn match_alts(ir: Arc<AntlrIR>, alts: &Vec<HashArc<AltIR>>) -> MatchNode {
    let mut match_node = match_element(alts[0].clone(), 0, ir.clone());

    for next_alt in alts.get(1..).unwrap() {
        match_node.merge(match_element(next_alt.clone(), 0, ir.clone()), ir.clone())
    };

    match_node
}

#[instrument(skip(ir))]
pub fn match_rule(ir: Arc<AntlrIR>, rule: usize) -> MatchNode {
    let rule = ir.rules().get(rule).unwrap();

    match_alts(ir.clone(), rule.alts())
}