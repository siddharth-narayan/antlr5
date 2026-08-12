use bimap::BiMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::{
    antlr::ast::EBNFSuffix,
    codegen::{
        RuleRef,
        intermediate::{
            AntlrIR,
            alt::AltIR,
            element::{AtomIR, ElementIR},
        },
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Choice {
    pub tree: HashMap<AtomIR, MatchNode>,
}

impl Choice {
    pub fn new(tree: HashMap<AtomIR, MatchNode>) -> Choice {
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

impl AntlrIR {
    // Lookahead and nth functions do not save their state for any alt, so likely could be a lot of performance gain saving the NTH set for each alt
    pub fn nth(&mut self, alt: Arc<AltIR>, n: usize) -> BiMap<AtomIR, usize> {
        let mut visited = HashSet::new();
        self.internal_nth(alt, 0, n, 0, &mut visited)
    }

    // This function requires polonius to correctly understand control flow's lifetime situation
    fn internal_nth(
        &self,
        alt: Arc<AltIR>,
        starting_element: usize,

        n: usize,
        depth: usize,
        visited: &mut HashSet<usize>,
    ) -> BiMap<AtomIR, usize> {
        let mut pos = 0;
        let mut nth_atoms = BiMap::new();

        let elements = match alt.elements().get(starting_element..) {
            Some(e) => e,
            None => return nth_atoms
        };

        for (element_index, element) in elements.iter().enumerate() {
            match element {
                ElementIR::Atom(atom) => {
                    match atom.suffix() {
                        Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Star) => {
                            nth_atoms.extend(self.internal_nth(alt.clone(), starting_element + element_index + 1, n, depth + 1, visited));
                        },
                        _ => ()
                    };

                    if let AtomIR::RuleID { id, .. } = atom {
                        if visited.contains(id) {
                            continue;
                        } else {
                            visited.insert(*id);
                        }

                        for alt in self.get_rule(*id).unwrap().alts().clone() {
                            nth_atoms.extend(
                                self.internal_nth(alt, 0, n - pos, depth + 1, visited).clone(),
                            )
                        }
                    }

                    if pos >= n {
                        nth_atoms.insert(atom.clone(), depth);
                        break;
                    }

                    pos += 1;
                }

                ElementIR::Block { block, suffix: _ } => {
                    for alt in block {
                        nth_atoms.extend(
                            self.internal_nth(alt.clone(), 0, n - pos, depth + 1, visited)
                                .clone(),
                        )
                    }
                    // Nth needs to continue here, reading anything that follows
                }
            }
        }

        nth_atoms
    }

    // Instead of lookahead we make match paths for an alt + element index
    pub fn lookahead(&mut self, rule: usize) -> MatchNode {
        let alts = self
            .rules()
            .get(rule)
            .unwrap()
            .alts().clone();

        // println!("Lookahead for rule {}", rule);
        self.internal_lookahead_alts(&alts)
    }

    pub fn internal_lookahead_alts(&mut self, alts: &Vec<Arc<AltIR>>) -> MatchNode {

        let alts = alts.iter()
            .enumerate()
            .map(|(index, alt)| (index, alt.clone()))
            .collect();
        
        self.internal_match_alts_enumerated(alts, 0)
    }

    // This function takes a set of alts, and their alt number, then calculates the approprate lookahead for deciding between alts
    // #[instrument(skip(self))]
    pub fn internal_match_alts_enumerated(
        &mut self,
        alts: HashMap<usize, Arc<AltIR>>,
        lookahead: usize,
    ) -> MatchNode {
        if alts.len() == 1 {
            return MatchNode::Terminal {
                alt: *alts.iter().nth(0).unwrap().0,
                continue_from: lookahead,
            };
        }

        // A map of atom's in the nth place to a map of alts and their depths 
        let mut first: HashMap<AtomIR, HashMap<usize, usize>> = HashMap::new();
        
        for (alt_index, alt) in &alts {
            let set = self.nth(alt.clone(), lookahead);
            if set.len() == 0 {
                continue; // Add FOLLOW sets. Right now whatever alt is longest will be matched
            }

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
}