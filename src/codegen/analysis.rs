use serde::{Deserialize, Serialize};
use tracing::instrument;
use std::{
    collections::{HashMap, HashSet, VecDeque, hash_set::IntoIter}, hash::Hash, sync::Arc,
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
    Choice(Choice),
    Atom {
        next: Option<Box<MatchNode>>
    },
}

pub struct HashSetMap<K: Hash + Eq, V: Hash + Eq> {
    map: HashMap<K, HashSet<V>>
}

impl<K: Debug + Hash + Eq, V: Debug + Hash + Eq> Debug for HashSetMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.map)
    }
}

impl<K: Hash + Eq + Clone, V: Hash + Eq> HashSetMap<K, V> {
    pub fn new() -> HashSetMap<K, V> {
        HashSetMap { map: HashMap::new() }
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

    pub fn extend<I: IntoIterator<Item = V>>(&mut self, key: K, values: I) -> Option<&HashSet<V>> {
        for item in values.into_iter() {
            self.insert(key.clone(), item);
        }

        self.get(&key)
    }
}

#[instrument(skip(nth_set_cache, visited, rules))]
pub fn nth<'a>(
    n:  usize,
    current_idx: usize,

    (alt, element_idx): (Arc<AltIR>, usize),
    continuation: &mut VecDeque<(Arc<AltIR>, usize)>,

    nth_set_cache: &'a mut HashSetMap<(Arc<AltIR>, usize), ElementIR>,
    visited: &mut HashSet<(Arc<AltIR>, usize, usize, usize)>,
    rules: &Vec<Arc<RuleIR>>,
) -> Option<&'a HashSet<ElementIR>> {
    // println!("\nn: {}, current_idx: {}, element_idx: {}", n, current_idx, element_idx);

    let mut set = HashSet::new();
        
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


fn element_match(alt: Arc<AltIR>, element_idx: usize) {

}

fn lookahead(
        ir: Arc<AntlrIR>,
        alts: HashMap<usize, Arc<AltIR>>,
        lookahead: usize,
    ) -> MatchNode {
        if alts.len() == 1 {
            return MatchNode::Terminal {
                alt: *alts.iter().nth(0).unwrap().0,
                continue_from: lookahead,
            };
        }

        // A map of tokens in the nth place to a set of alts 
        let mut tokenmap: HashSetMap<usize, usize> = HashSetMap::new();
        
        for (alt_index, alt) in &alts {
            let set = nth(lookahead, 0, (alt.clone(), 0), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::new(), ir.rules());
            if set.is_none_or(|s| s.len() == 0) {
                continue; // Add FOLLOW sets. Right now whatever alt is longest will be matched
            }

            let set = set.unwrap();
            
            for (atom, depth) in set {




                if first.get(&atom).is_none() {
                    first.insert(atom.clone(), HashMap::new());
                }

                let vec = first.get_mut(&atom).unwrap();
                vec.insert(*alt_index, depth);
            }
        }

        let mut out = HashMap::new();

        for (atom, available_alts) in &first {
            let mut filtered_alts: HashMap<usize, Arc<AltIR>> = HashMap::new();

            for alt in alts.iter().filter(|f| available_alts.contains(f.0)) {
                filtered_alts.insert(*alt.0, alt.1.clone());
            }

            out.insert(
                atom.clone(),
                self.internal_match_alts_enumerated(filtered_alts, lookahead + 1),
            );
        }

        MatchNode::Continues(Choice { tree: out })
    }