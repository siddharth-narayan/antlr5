use rapidhash::fast::RandomState;
use tracing::{error, instrument};
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

    if element.size().is_optional {
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