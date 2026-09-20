use rapidhash::fast::RandomState;
use tracing::instrument;
use std::{
    char, collections::{HashMap, HashSet, VecDeque, hash_set::IntoIter}, hash::Hash, mem::discriminant, sync::Arc,
};
use std::fmt::Debug;
use crate::{
    antlr::ast::{EBNFSuffix, Element}, codegen::{
        RuleRef, intermediate::{
            AntlrIR, alt::{self, AltIR}, element::{self, ElementIR}, rule::RuleIR,
        },
    }, util::{HashArc, HashSetMap},
};

// #[instrument(skip(nth_set_cache, visited, rules))]
pub fn nth<'a>(
    n:  usize,
    current_idx: usize,

    (alt, element_idx): (HashArc<AltIR>, usize),
    continuation: &mut VecDeque<(HashArc<AltIR>, usize)>,

    nth_set_cache: &'a mut HashSetMap<(HashArc<AltIR>, usize), ElementIR, RandomState>,
    visited: &mut HashSet<(HashArc<AltIR>, usize, usize, usize), RandomState>,
    rules: &Vec<RuleIR>,
) -> Option<&'a HashSet<ElementIR, RandomState>> {

    let mut set = HashSet::with_hasher(RandomState::new());
    
    let too_far = current_idx > n;
    let already_visited = !visited.insert((alt.clone(), n, element_idx, current_idx));
    if too_far || already_visited {
        return None
    }
    
    let element = match alt.elements().get(element_idx) {
        Some(e) => {
            e.clone()
        },
        None => {
            if let Some((continue_alt, continue_element_idx)) = continuation.pop_back() {
                set.extend(nth(n, current_idx, (continue_alt, continue_element_idx), continuation, nth_set_cache, visited, rules).into_flat_iter().cloned());
            }

            return nth_set_cache.extend((alt.clone(), element_idx), set.into_iter());
        }
    };

    // Overhangs not included here
    if current_idx == n {
        set.insert(element.clone());
    }

    if let Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Star) = element.suffix() {
        // Should continuation be cloned here?
        set.extend(nth(n, current_idx, (alt.clone(), element_idx + 1), continuation, nth_set_cache, visited, rules).into_flat_iter().cloned())
    };

    match element {
        ElementIR::TokenAtom { .. } => {
            set.extend(nth(n, current_idx + 1, (alt.clone(), element_idx + 1), continuation, nth_set_cache, visited, rules).into_flat_iter().cloned())
        }

        ElementIR::RuleAtom { id, .. } => {
            for rule_alt in rules.get(id).unwrap().alts().clone() {
                continuation.push_back((alt.clone(), element_idx + 1));
                set.extend(nth(n, current_idx, (rule_alt.clone(), 0), continuation, nth_set_cache, visited, rules).into_flat_iter().cloned())
            }
        }

        _ => ()
    }

    // Do we need this?
    return nth_set_cache.extend((alt.clone(), element_idx), set.into_iter())
}

#[derive(Hash, Clone, Debug, PartialEq, Eq)]
struct PeekMatch {
    pub item: usize, // Can represent a character or token
    pub greedy: bool, // Should we consume one, or as many as possible
    pub can_become_greedy: bool, // If it comes from an element with a greedy suffix
}

impl PeekMatch {
    pub fn from_token_element(element: &ElementIR) -> PeekMatch {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MatchNode {
    Peek {
        // alt: HashArc<AltIR>,
        // element_idx: usize,

        peek: HashMap<usize, MatchNode, RandomState>,
        fallback: Option<Box<MatchNode>>
    },

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
    #[instrument(skip(self, ir, other))]
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
        let MatchNode::Peek { peek: peek_left, fallback: fallback_left } = self else { panic!() };
        let MatchNode::Peek { peek: peek_right, fallback: fallback_right } = other else { panic!() };

        for (token_right, node_right) in peek_right {
                match peek_left.get_mut(&token_right) {
                    Some(match_left) => {
                        match_left.merge(node_right, ir.clone());
                    },
                    None => {
                        peek_left.insert(token_right, node_right);
                    }
                }
            }

            // Match fallback
            match fallback_left {
                Some(fallback_left) => {
                    if let Some(fallback_right) = fallback_right {
                        fallback_left.merge(*fallback_right, ir);
                    };
                },
                None => {
                    *fallback_left = fallback_right
                }
            };
    }

    pub fn merge_peek_element(&mut self, other: MatchNode, ir: Arc<AntlrIR>) {
        let MatchNode::Peek { peek: peek_left, fallback: fallback_left } = self else { panic!() };
        let MatchNode::Element { alt, element_idx, element, next } = &other else { panic!() };
        
        match peek_left.get_mut(&element.id().unwrap_or_else(|| { println!("{:#?}", element); panic!() })) {
            Some(match_left) => {
                match_left.merge(other, ir);
            },
            None => {
                // Do we insert other here?
                peek_left.insert(element.id().unwrap(), other);
            }
        }
    }

    pub fn merge_element_element(&mut self, other: MatchNode, ir: Arc<AntlrIR>) {
        let MatchNode::Element { alt: alt_left, element_idx: element_idx_left, element: element_left, next: next_left } = &other else { panic!() };
        let MatchNode::Element { alt: alt_right, element_idx: element_idx_right, element: element_right, next: next_right } = &other else { panic!() };
        
        let mut peek = HashMap::default();
        if let Some(nthset) = nth(0, 0, (alt_left.clone(), *element_idx_left), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::default(), ir.rules()) {
            for element in nthset {
                if let ElementIR::TokenAtom { .. } = element {
                    peek.insert(element.id().unwrap(), match_element_dumb(alt_left.clone(), *element_idx_left, ir.clone()));
                }
            }
        }
        
        if let Some(nthset) = nth(0, 0, (alt_right.clone(), *element_idx_right), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::default(), ir.rules()) {
            for element in nthset {
                if let ElementIR::TokenAtom { .. } = element {
                    peek.insert(element.id().unwrap(), match_element_dumb(alt_right.clone(), *element_idx_right, ir.clone()));
                }
            }
        }
        
        *self = MatchNode::Peek { peek, fallback: None };
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

    if element.suffix() == Some(EBNFSuffix::Optional) ||  element.suffix() == Some(EBNFSuffix::Star) {
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