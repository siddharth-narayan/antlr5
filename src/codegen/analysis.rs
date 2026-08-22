use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, hash_set::IntoIter}, hash::Hash, sync::Arc,
};

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

struct Cache<K: Hash + Eq, V: Hash + Eq> {
    map: HashMap<K, HashSet<V>>
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

// Lookahead and nth functions do not save their state for any alt, so likely could be a lot of performance gain saving the NTH set for each alt
// pub fn nth(rules: &Vec<Arc<RuleIR>>, alt: Arc<AltIR>, n: usize) -> Option<HashSet<ElementIR>> {
//     let mut n_cache = Cache::new();
//     let mut len_cache = Cache::new();

//     let mut visited = HashSet::new();
//     internal_nth(&mut n_cache, &mut len_cache, rules, alt, 0, n, 0, &mut visited)
// }

fn alt_len(alt: Arc<AltIR>) {

}

fn internal_nth<'a>(
    alt: Arc<AltIR>,
    n:  usize,

    current_pos: usize,
    element_pos: usize,
    nth_cache: &'a mut Cache<(Arc<AltIR>, usize), ElementIR>,
    rules: &Vec<Arc<RuleIR>>,
) -> Option<&'a HashSet<ElementIR>> {
    if let Some(e) = nth_cache.get(&(alt.clone(), n)) {
        return Some(e)
    }

    let element_ref = alt.clone();
    let element = element_ref.elements().get(element_pos)?;

    if let ElementIR::TokenAtom { .. } | ElementIR::RuleAtom { .. } = element {
        if current_pos >= n {
            nth_cache.insert((alt.clone(), n), element.clone());
            return nth_cache.get(&(alt, n))
        }
    }

    if let Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Star) = element.suffix() {
        let optional_skipped: HashSet<ElementIR> = internal_nth(alt.clone(), n, current_pos + 1, element_pos + 1, nth_cache, rules).into_flat_iter().cloned().collect();
        nth_cache.extend((alt.clone(), n), optional_skipped);
    };

    match element {
        ElementIR::RuleAtom { id, .. } => {
            for alt in rules.get(*id).unwrap().alts().clone() {
                let alt_nth: HashSet<ElementIR> = internal_nth(alt.clone(), n, current_pos + 1,0, nth_cache, rules).into_flat_iter().cloned().collect();
                nth_cache.extend((alt.clone(), n).clone(), alt_nth)
            }
        }

        ElementIR::Block { block, .. } => {
            for alt in block.clone() {
                let alt_nth: HashSet<ElementIR> = internal_nth(alt.clone(), n, current_pos, 0, nth_cache, rules).into_flat_iter().cloned().collect();
                nth_cache.extend((alt.clone(), n).clone(), alt_nth);
            }
        }

        _ => ()
    }

    return internal_nth(alt, n, current_pos + 1 , element_pos + 1, nth_cache, rules)
}