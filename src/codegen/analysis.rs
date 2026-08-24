use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, hash_set::IntoIter}, hash::Hash, sync::Arc,
};
use std::fmt::Debug;
use crate::{
    antlr::ast::EBNFSuffix, codegen::{
        RuleRef, intermediate::{
            AntlrIR, alt::{self, AltIR}, element::ElementIR, rule::RuleIR,
        },
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Choice {
    pub tree: HashMap<ElementIR, MatchNode>,
}

impl Choice {
    pub fn new(tree: HashMap<ElementIR, MatchNode>) -> Choice {
        Choice { tree }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MatchNode {
    Continues(Choice),
    Atom {
        next: Option<Box<MatchNode>>
    },
}

pub struct Cache<K: Hash + Eq, V: Hash + Eq> {
    map: HashMap<K, HashSet<V>>
}

impl<K: Debug + Hash + Eq, V: Debug + Hash + Eq> Debug for Cache<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.map)
    }
}

impl<K: Hash + Eq + Clone, V: Hash + Eq> Cache<K, V> {
    pub fn new() -> Cache<K, V> {
        Cache { map: HashMap::new() }
    }

    pub fn get(&self, key: &K) -> Option<&HashSet<V>> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: K, item: V) {
        match self.map.get_mut(&key) {
            Some(s) => {
                s.insert(item);
            },

            None => {
                let mut set = HashSet::new();
                set.insert(item);
                self.map.insert(key, set);
            }
        }
    }

    pub fn extend<I: IntoIterator<Item = V>>(&mut self, key: K, values: I) {
        for item in values.into_iter() {
            self.insert(key.clone(), item);
        }
    }
}

pub fn alt_len<'a>(alt: Arc<AltIR>, element_pos: usize, count: usize, visited: &mut HashSet<(Arc<AltIR>, usize)>, len_cache: &mut HashSet<usize>, rules: &Vec<Arc<RuleIR>>) {
    if !visited.insert((alt.clone(), element_pos)) {
        return
    }

    let element = match alt.elements().get(element_pos) {
        Some(e) => e,
        None => {
            // Where the magic happens
            len_cache.insert(count - 1);
            return;
        }
    };
    
    if let Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Star) = element.suffix() {
        alt_len(alt.clone(), element_pos + 1, count, visited, len_cache, rules);
    };

    match element {
        ElementIR::RuleAtom { id, .. } => {
            for rule_alt in rules.get(*id).unwrap().alts() { // same problem it fails after not finishing the rest of the current rule
                alt_len(rule_alt.clone(), 0, count, visited, len_cache, rules);
            }
        },
        ElementIR::TokenAtom { .. } => {
            alt_len(alt.clone(), element_pos + 1, count + 1, visited, len_cache, rules);
        },
        ElementIR::Block { block, .. } => {
            for rule_alt in block {
                alt_len(rule_alt.clone(), 0, count, visited, len_cache, rules);
            }
        }
        _ => ()
    }
}

pub fn nth<'a>(
    n:  usize,
    current_idx: usize,

    (alt, element_idx): (Arc<AltIR>, usize),
    continuation: Option<(Arc<AltIR>, usize)>,

    nth_set_cache: &'a mut Cache<(Arc<AltIR>, usize), ElementIR>,
    visited: &mut HashSet<(Arc<AltIR>, usize, usize)>,
    rules: &Vec<Arc<RuleIR>>,
) -> Option<&'a HashSet<ElementIR>> {
    let mut set = HashSet::new();
    
    if current_idx > n || !visited.insert((alt.clone(), n, element_idx)) {
        return None;
    }
    
    let element = alt.elements().get(element_idx)?.clone();

    if current_idx == n {
        nth_set_cache.insert((alt.clone(), n), element.clone());
    }

    if let Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Star) = element.suffix() {
        set.extend(nth(n, current_idx, (alt.clone(), element_idx + 1), None, nth_set_cache, visited, rules).into_flat_iter().cloned())
    };

    match element {
        ElementIR::RuleAtom { id, .. } => {
            for rule_alt in rules.get(id).unwrap().alts().clone() {
                set.extend(nth(n, current_idx, (rule_alt.clone(), 0), Some((alt.clone(), element_idx + 1)), nth_set_cache, visited, rules).into_flat_iter().cloned())
            }
        }

        ElementIR::Block { block, .. } => {
            for rule_alt in block {
                set.extend(nth(n, current_idx, (rule_alt.clone(), 0), Some((alt.clone(), element_idx + 1)), nth_set_cache, visited, rules).into_flat_iter().cloned())
            }
        }

        _ => ()
    }

    if let Some((continue_alt, continue_element_idx)) = continuation {
        set.extend(nth(n, current_idx + 1, (continue_alt, continue_element_idx), None, nth_set_cache, visited, rules).into_flat_iter().cloned());
    }

    nth_set_cache.extend((alt.clone(), element_idx), set.into_iter());
    Some(nth_set_cache.get(&(alt, element_idx)).unwrap())
}