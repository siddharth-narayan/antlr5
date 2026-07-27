use std::collections::{HashMap, HashSet};
use bimap::BiMap;

use crate::{antlr::ast::EBNFSuffix, codegen::intermediate::{AntlrIR, alt::AltIR, element::{AtomIR, ElementIR}}};

#[derive(Debug)]
pub struct LookAhead<'a> {
    pub tree: HashMap<&'a AtomIR, LookAheadNode<'a>>
}

impl<'a> LookAhead<'a> {
    pub fn new(tree: HashMap<&'a AtomIR, LookAheadNode<'a>>) -> LookAhead<'a> {
        LookAhead { tree }
    }
}

#[derive(Debug)]
pub enum LookAheadNode<'a> {
    Continues(LookAhead<'a>),
    Terminal {
        alt: usize, // The alt to pick
        continue_from: usize // The element that needs to next be matched
    }
}

impl AntlrIR {
    pub fn rule_always_contains(&self, rule: usize, should_contain: usize) -> bool {
        let mut visited = HashSet::new();
        self.internal_rule_always_contains(rule, should_contain, &mut visited)
    }

    fn internal_rule_always_contains(&self, rule: usize, should_contain: usize, visited: &mut HashSet<usize>) -> bool {
        if !visited.insert(rule) {
            return true;
        }

        let rule = self.rules.get(rule).expect("expexted rule");

        for alt in rule.alts() {
            if !self.internal_alt_always_contains(&alt, should_contain, visited) {
                return false;
            }
        };

        true
    }

    fn internal_alt_always_contains(&self, alt: &AltIR, should_contain: usize, visited: &mut HashSet<usize>) -> bool {
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

                        if !visited.contains(id) && self.internal_rule_always_contains(*id, should_contain, visited) {
                            return true
                        }
                    }
                },

                ElementIR::Block { block, suffix } => {
                    if let Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Star) = *suffix {
                        continue;
                    }

                    if block.iter().all(|alt| self.internal_alt_always_contains(alt, should_contain, visited)) {
                        return true
                    }
                }
            }
        };

        false
    }

    // Lookahead and nth functions do not save their state for any alt, so likely could be a lot of performance gain saving the NTH set for each alt
    pub fn nth<'a>(&'a self, alt: &'a AltIR, n: usize) -> BiMap<&'a AtomIR, usize> {
        let mut visited = HashSet::new();
        self.internal_nth(alt, n, 0, &mut visited)
    }

    fn internal_nth<'a>(&'a self, alt: &'a AltIR, mut n: usize, depth: usize, visited: &mut HashSet<usize>) -> BiMap<&'a AtomIR, usize> {
        let mut nth_atoms = BiMap::new();

        for element in alt.elements() {
            match element {
                ElementIR::Atom { atom, suffix: _ } => {
                    if n == 0 {
                        let _ = nth_atoms.insert_no_overwrite(atom, depth);

                        if let AtomIR::RuleID(id) = atom {
                            if visited.contains(id) {
                                continue;
                            }

                            visited.insert(*id);

                            let rule = self.rules.get(*id).unwrap();
                            for alt in rule.alts() {
                                nth_atoms.extend(self.internal_nth(alt, n, depth + 1, visited))
                            }
                        }

                        return nth_atoms;
                    } else {
                        // Need to handle optional rules here
                        if let AtomIR::RuleID(id) = atom {
                            if visited.contains(id) {
                                continue;
                            }

                            visited.insert(*id);

                            let rule = self.rules.get(*id).unwrap();
                            for alt in rule.alts() {
                                nth_atoms.extend(self.internal_nth(alt, n, depth + 1, visited))
                            }
                        }

                        n -= 1;
                    }
                },
                ElementIR::Block { block, suffix: _ } => {
                    for alt in block {
                        nth_atoms.extend(self.internal_nth(alt, n, depth + 1, visited))
                    }
                    // Nth needs to continue here, reading anything that follows
                }
            }
        }

        nth_atoms
    }

    pub fn lookahead(&self, rule: usize) -> LookAheadNode<'_> {
        let alts: HashMap<usize, &AltIR> = self.rules().get(rule).unwrap().alts().iter().enumerate().collect();
        // println!("Lookahead for rule {}", rule);
        self.lookahead_alts(alts, 0)   
    }

    // This function takes a set of alts, and their alt number, then calculates the approprate lookahead for deciding between alts
    pub fn lookahead_alts<'a>(&'a self, alts: HashMap<usize, &'a AltIR>, lookahead: usize) -> LookAheadNode<'a> {
        // println!("Lookahead for alts {:#?} with {} lookahead", alts.iter().map(|a| a.0).collect::<Vec<usize>>(), lookahead);

        if alts.len() == 0 {

        }
        
        if alts.len() == 1 {
            return LookAheadNode::Terminal { alt: *alts.iter().nth(0).unwrap().0, continue_from: lookahead }
        }

        let mut first: HashMap<&AtomIR, HashSet<usize>> = HashMap::new();
        for (index, alt) in &alts {
            let set = self.nth(alt, lookahead);
            for (atom, _depth) in set {
                match first.get_mut(atom) {
                    Some(vec) => {
                        vec.insert(*index);
                    },
                    None => {
                        let mut s = HashSet::new();
                        s.insert(*index);
                        first.insert(atom, s);
                    }
                }
            }
        };

        let mut out = HashMap::new();

        for (atom, available_alts) in first {
            let mut filtered_alts: HashMap<usize, &AltIR> = HashMap::new();
            for alt in alts.iter().filter(|f| available_alts.contains(f.0)) {
                filtered_alts.insert(*alt.0, alt.1);
            }

            out.insert(atom, self.lookahead_alts(filtered_alts, lookahead + 1));
        }

        LookAheadNode::Continues(LookAhead { tree: out })
    }
}