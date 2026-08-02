use bimap::BiMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tracing::{Level, event, instrument};

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
pub struct LookAhead {
    pub tree: HashMap<AtomIR, LookAheadNode>,
}

impl LookAhead {
    pub fn new(tree: HashMap<AtomIR, LookAheadNode>) -> LookAhead {
        LookAhead { tree }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum LookAheadNode {
    Continues(LookAhead),
    Terminal {
        alt: usize,           // The alt to pick
        continue_from: usize, // The element that needs to next be matched
    },
}

impl AntlrIR {
    pub fn rule_always_contains(&self, rule: usize, should_contain: usize) -> bool {
        let mut visited = HashSet::new();
        self.internal_rule_always_contains(rule, should_contain, &mut visited)
    }

    fn internal_rule_always_contains(
        &self,
        rule: usize,
        should_contain: usize,
        visited: &mut HashSet<usize>,
    ) -> bool {
        if !visited.insert(rule) {
            return true;
        }

        let rule = self.rules().get(rule).expect("expexted rule");

        for alt in rule.alts() {
            if !self.internal_alt_always_contains(&alt, should_contain, visited) {
                return false;
            }
        }

        true
    }

    fn internal_alt_always_contains(
        &self,
        alt: &AltIR,
        should_contain: usize,
        visited: &mut HashSet<usize>,
    ) -> bool {
        // This might not always be correct for recursive alts -- but because we only use it in internal_rule_always_contains, the result of THAT function will always be correct.
        // This shoudl be fixed in the future, but it's annoying to deal with so I'm leaving it like this
        if alt.is_recursive() {
            return true;
        }

        for element in alt.elements() {
            match element {
                ElementIR::Atom { atom, suffix } => {
                    if let Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Star) = *suffix {
                        continue;
                    }

                    if let AtomIR::RuleID(id) = atom {
                        if *id == should_contain {
                            return true;
                        }

                        if !visited.contains(id)
                            && self.internal_rule_always_contains(*id, should_contain, visited)
                        {
                            return true;
                        }
                    }
                }

                ElementIR::Block { block, suffix } => {
                    if let Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Star) = *suffix {
                        continue;
                    }

                    if block
                        .iter()
                        .all(|alt| self.internal_alt_always_contains(alt, should_contain, visited))
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    // Lookahead and nth functions do not save their state for any alt, so likely could be a lot of performance gain saving the NTH set for each alt
    pub fn nth<'a>(&mut self, alt: Arc<AltIR>, n: usize) -> BiMap<AtomIR, usize> {
        let mut visited = HashSet::new();
        self.internal_nth(alt, n, 0, &mut visited)
    }

    pub fn rule_nth(&mut self, rule: usize, n: usize) -> BiMap<AtomIR, usize> {
        let mut result = BiMap::new();
        for alt in self.rules().get(rule).unwrap().alts().clone() {
            let alt = alt.clone();

            result.extend(self.nth(alt, n).clone());
        };

        result
    }

    // This function requires polonius to correctly understand control flow's lifetime situation
    fn internal_nth(
        &self,
        alt: Arc<AltIR>,
        n: usize,
        depth: usize,
        visited: &mut HashSet<usize>,
    ) -> BiMap<AtomIR, usize> {
        let mut pos = 0;
        let mut nth_atoms = BiMap::new();

        for element in alt.elements() {
            match element {
                ElementIR::Atom { atom, suffix } => {
                    if let AtomIR::RuleID(id) = atom {
                        if visited.contains(id) {
                            continue;
                        } else {
                            visited.insert(*id);
                        }

                        for alt in self.get_rule(*id).unwrap().alts().clone() {
                            nth_atoms.extend(
                                self.internal_nth(alt, n - pos, depth + 1, visited).clone(),
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
                            self.internal_nth(alt.clone(), n - pos, depth + 1, visited)
                                .clone(),
                        )
                    }
                    // Nth needs to continue here, reading anything that follows
                }
            }
        }

        nth_atoms
    }

    pub fn lookahead(&mut self, rule: usize) -> LookAheadNode {
        let alts = self
            .rules()
            .get(rule)
            .unwrap()
            .alts().clone();

        // println!("Lookahead for rule {}", rule);
        self.internal_lookahead_alts(&alts)
    }

    pub fn internal_lookahead_alts(&mut self, alts: &Vec<Arc<AltIR>>) -> LookAheadNode {

        let alts = alts.iter()
            .enumerate()
            .map(|(index, alt)| (index, alt.clone()))
            .collect();
        
        self.internal_lookahead_alts_enumerated(alts, 0)
    }

    // This function takes a set of alts, and their alt number, then calculates the approprate lookahead for deciding between alts
    // #[instrument(skip(self))]
    pub fn internal_lookahead_alts_enumerated<'a>(
        &mut self,
        alts: HashMap<usize, Arc<AltIR>>,
        lookahead: usize,
    ) -> LookAheadNode {
        // Its nth sets being cached with the wrong n
        // println!("{} lookahead", lookahead);
        if alts.len() == 0 {
            // panic!("altlen0")
        }

        if alts.len() == 1 {
            return LookAheadNode::Terminal {
                alt: *alts.iter().nth(0).unwrap().0,
                continue_from: lookahead,
            };
        }

        let mut first: HashMap<AtomIR, HashSet<usize>> = HashMap::new();
        for (index, alt) in &alts {
            let set = self.nth(alt.clone(), lookahead);
            // println!("alt {} with {} lookahead has set size {}: {:#?}", index, lookahead, set.len(), set);
            if set.len() == 0 {
                continue; // Add FOLLOW sets. Right now whatever alt is longest will be matched
                // return LookAheadNode::Terminal { alt: usize::MAX, continue_from: usize::MAX }
            }
            for (atom, _depth) in set {
                if let AtomIR::RuleID(_) = atom {
                    continue;
                }
                
                match first.get_mut(&atom) {
                    Some(vec) => {
                        vec.insert(*index);
                    }
                    None => {
                        let mut s = HashSet::new();
                        s.insert(*index);
                        first.insert(atom.clone(), s);
                    }
                }
            }
        }

        // panic!("{:#?}", first);
        let mut out = HashMap::new();

        for (atom, available_alts) in first {
            let mut filtered_alts: HashMap<usize, Arc<AltIR>> = HashMap::new();
            for alt in alts.iter().filter(|f| available_alts.contains(f.0)) {
                filtered_alts.insert(*alt.0, alt.1.clone());
            }

            out.insert(
                atom.clone(),
                self.internal_lookahead_alts_enumerated(filtered_alts, lookahead + 1),
            );
        }

        LookAheadNode::Continues(LookAhead { tree: out })
    }
}
