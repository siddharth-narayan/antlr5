use rapidhash::fast::RandomState;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use std::{
    collections::{HashMap, HashSet, VecDeque, hash_set::IntoIter}, hash::Hash, mem::discriminant, sync::Arc,
};
use std::fmt::Debug;
use crate::{
    antlr::ast::{EBNFSuffix, Element}, codegen::{
        RuleRef, intermediate::{
            AntlrIR, alt::{self, AltIR}, element::{self, ElementIR}, rule::RuleIR,
        },
    }, util::HashSetMap,
};

#[instrument(skip(nth_set_cache, visited, rules))]
pub fn nth<'a>(
    n:  usize,
    current_idx: usize,

    (alt, element_idx): (Arc<AltIR>, usize),
    continuation: &mut VecDeque<(Arc<AltIR>, usize)>,

    nth_set_cache: &'a mut HashSetMap<(Arc<AltIR>, usize), ElementIR, RandomState>,
    visited: &mut HashSet<(Arc<AltIR>, usize, usize, usize), RandomState>,
    rules: &Vec<Arc<RuleIR>>,
) -> Option<&'a HashSet<ElementIR, RandomState>> {
    // println!("\nn: {}, current_idx: {}, element_idx: {}", n, current_idx, element_idx);

    let mut set = HashSet::with_hasher(RandomState::new());
        
    let too_far = current_idx > n;
    let already_visited = !visited.insert((alt.clone(), n, element_idx, current_idx));
    if too_far || already_visited {
        return None
    }
    
    let element = match alt.elements().get(element_idx) {
        Some(e) => {
            // println!("{:#?}", e);
            e.clone()
        },
        None => {
            if let Some((continue_alt, continue_element_idx)) = continuation.pop_back() {
                set.extend(nth(n, current_idx, (continue_alt, continue_element_idx), continuation, nth_set_cache, visited, rules).into_flat_iter().cloned());
            }

            // println!("set for alt {:#?}: {:#?}", alt, set);
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

        ElementIR::Block { block, .. } => {
            for rule_alt in block {
                continuation.push_back((alt.clone(), element_idx + 1));
                set.extend(nth(n, current_idx, (rule_alt.clone(), 0), continuation, nth_set_cache, visited, rules).into_flat_iter().cloned())
            }
        }

        _ => ()
    }

    // Do we need this?
    // println!("set for alt {:#?}: {:#?}", alt, set);
    return nth_set_cache.extend((alt.clone(), element_idx), set.into_iter())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MatchNode {
    Peek {
        alt: Arc<AltIR>,
        element_idx: usize,

        peek: HashMap<usize, MatchNode, RandomState>,
        fallback: Option<Box<MatchNode>>
    },

    Element {
        alt: Arc<AltIR>,
        element_idx: usize,

        element: ElementIR,
        next: Option<Box<MatchNode>>
    },

    Finish,
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
    pub fn merge(&mut self, other: MatchNode) {
        match self {
            MatchNode::Peek { alt: alt_left, element_idx: element_idx_left, peek: peek_left,fallback: fallback_left } => {
                match other {
                    MatchNode::Peek { alt: alt_right, element_idx: element_idx_right, peek: peek_right, fallback: fallback_right } => {
                        // Match fallback
                        *fallback_left = fallback_left.map_or(
                            fallback_right,
                            |mut fallback_left: Box<MatchNode>| {
                            if let Some(fallback_right) = fallback_right {
                                fallback_left.merge(*fallback_right);
                            }

                            Some(fallback_left)
                        });
                    },
                    MatchNode::Element { alt, element_idx, element, next } => todo!(),
                    MatchNode::Finish => todo!(),
                }
            },

            MatchNode::Element { alt, element_idx, element, next } => {
                match other {
                    // Swap and recurse
                    MatchNode::Peek { alt, element_idx, peek } => {
                        
                    },
                    MatchNode::Element { alt, element_idx, element, next } => todo!(),
                    MatchNode::Finish => todo!(),
                }
            }
            
            MatchNode::Finish => {
                *self = other;
            }
        }
    }
}

fn match_element(alt: Arc<AltIR>, element_idx: usize) -> Option<MatchNode> {
    Some(MatchNode::Element { alt: alt.clone(), element_idx, element: alt.elements().get(element_idx)?.clone(), next: match_element(alt, element_idx + 1).map(Box::new) })
}

fn match_alts(alts: &Vec<Arc<AltIR>>) -> MatchNode {
    let mut match_node = match_element(alts[0].clone(), 0).unwrap();

    for next_alt in alts.get(1..).unwrap() {
        match_node.merge(match_element(next_alt.clone(), 0).unwrap())
    };

    match_node
}

// fn match_alts_old(
//         ir: Arc<AntlrIR>,
//         alts: HashMap<usize, (Arc<AltIR>, usize)>,
//         lookahead: usize,
//     ) -> MatchNode {
//         if alts.len() == 1 {
//             let (_, (alt, element_idx)) = alts.iter().nth(0).unwrap();
//             return match_element(alt.clone(), *element_idx);
//         }
        
//         let mut tokenmap: HashSetMap<ElementIR, Arc<AltIR>, RandomState> = HashSetMap::new();
        
//         for (_, (alt, _)) in &alts {
//             let mut stack = VecDeque::new();
//             let mut nth_cache = HashSetMap::new();

//             let set = nth(lookahead, 0, (alt.clone(), 0), &mut stack, &mut nth_cache, &mut HashSet::default(), ir.rules());
//             if set.is_none_or(|s| s.len() == 0) {
//                 continue; // Add FOLLOW sets. Right now whatever alt is longest will be matched
//             }

//             let set = set.unwrap().clone();

//             for element in set {
//                 if let ElementIR::TokenAtom { .. } = element {
//                     tokenmap.insert(element, alt.clone())
//                 }
//             };
//         }

//         let mut out = HashMap::default();

//         for (element, alts) in tokenmap.clone() {
//             let element_matches = |e: &ElementIR, mut _b: &mut HashSet<Arc<AltIR>, RandomState>| {
//                 println!("{:#?}", _b);
//                 discriminant(&element) == discriminant(e) && element.id().unwrap() == e.id().unwrap()
//             };

//             let ambiguous_matches: Vec<_> = tokenmap.remove_all_keys_matching(element_matches).collect();
            
            
//         }

//         MatchNode::Peek(out)
// }
