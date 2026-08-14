use bimap::BiMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, hash_set::IntoIter}, hash::Hash, sync::Arc,
};

use crate::{
    antlr::ast::EBNFSuffix, codegen::{
        RuleRef, intermediate::{
            AntlrIR, alt::AltIR, element::ElementIR, rule::RuleIR,
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

    pub fn get(&self, key: K) -> Option<&HashSet<V>> {
        self.map.get(&key)
    }

    pub fn insert(&mut self, key: &K, item: V) {
        match self.map.get_mut(key) {
            Some(s) => {
                s.insert(item);
            },

            None => {
                let mut set = HashSet::new();
                set.insert(item);
                self.map.insert(key.clone(), set);
            }
        }
    }

    pub fn extend<I: IntoIterator<Item = V>>(&mut self, key: &K, values: I) {
        for item in values.into_iter() {
            self.insert(key, item);
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

fn internal_nth<'a>(
    alt: Arc<AltIR>,
    n: usize,

    current_pos: usize,
    element_pos: usize,
    nth_cache: &'a mut Cache<(Arc<AltIR>, usize), ElementIR>,
    len_cache: &'a mut Cache<Arc<AltIR>, usize>,
    rules: &Vec<Arc<RuleIR>>,
    visited: &mut HashSet<usize>,
) -> Option<&'a HashSet<ElementIR>> {
    if let Some(e) = nth_cache.get((alt, n)) {
        return Some(e)
    }

    let element = alt.elements().get(element_pos)?;
    if let Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Star) = element.suffix() {
        let mut new_visited = visited.clone();
        nth_cache.extend(&(alt, n), internal_nth(alt.clone(), n, current_pos, element_pos + 1, nth_cache, len_cache, rules, &mut new_visited).into_flat_iter().cloned());
    };

    match element {
        ElementIR::RuleAtom { id, .. } => {
            if !visited.contains(id) {
                visited.insert(*id);

                for alt in rules.get(*id).unwrap().alts().clone() {
                    let mut new_visited = visited.clone();
                    nth_atoms.extend(
                        internal_nth(rules, alt, 0, n - pos, depth + 1, &mut new_visited).clone(),
                    )
                }
            }
        }

        ElementIR::Block { block, .. } => {
            for alt in block {
                let mut new_visited = visited.clone();
                nth_atoms.extend(
                    internal_nth(rules, alt.clone(), 0, n - pos, depth + 1, &mut new_visited)
                        .clone(),
                )
            }
        }

        _ => ()
    }

    if let ElementIR::TokenAtom { .. } | ElementIR::RuleAtom { .. } = element {
        if pos >= n {
            nth_atoms.insert(element.clone(), depth);
            break;
        }
    }

    nth_cache.get(key)
}