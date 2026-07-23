use std::collections::{HashMap, HashSet};
use bimap::BiMap;

use crate::codegen::intermediate::{AltIR, AntlrIR, AtomIR, ElementIR};

pub struct LookAhead<'a> {
    pub tree: HashMap<&'a AtomIR, LookAheadNode<'a>>
}

impl<'a> LookAhead<'a> {
    pub fn new(tree: HashMap<&'a AtomIR, LookAheadNode<'a>>) -> LookAhead<'a> {
        LookAhead { tree }
    }
}

pub enum LookAheadNode<'a> {
    Continues(LookAhead<'a>),
    Terminal {
        alt: usize, // The alt to pick
        continue_from: usize // The element that needs to next be matched
    }
}

impl AntlrIR {
    pub fn nth<'a>(&'a self, alt: &'a AltIR, n: usize) -> BiMap<&'a AtomIR, usize> {
        self.internal_nth(alt, n, 0)
    }

    fn internal_nth<'a>(&'a self, alt: &'a AltIR, mut n: usize, depth: usize) -> BiMap<&'a AtomIR, usize> {
        let mut set = BiMap::new();

        for element in alt.elements() {
            match element {
                ElementIR::Atom { atom, suffix: _ } => {
                    if n == 0 {
                        set.insert_no_overwrite(atom, depth);

                        if let AtomIR::RuleID(id) = atom {
                            let rule = self.rules.get(*id).unwrap();
                            for alt in rule.alts() {
                                set.extend(self.internal_nth(alt, n, depth + 1))
                            }
                        }

                        return set;
                    } else {
                        // Need to handle optional rules here
                        if let AtomIR::RuleID(id) = atom {
                            let rule = self.rules.get(*id).unwrap();
                            for alt in rule.alts() {
                                set.extend(self.internal_nth(alt, n, depth + 1))
                            }
                        }

                        n -= 1;
                    }
                },
                ElementIR::Block { block, suffix: _ } => {
                    for alt in block {
                        set.extend(self.nth(alt, n))
                    }
                    // Nth needs to continue here, reading anything that follows
                }
            }
        }

        set
    }

    pub fn lookahead(&self, rule: usize) -> LookAheadNode<'_> {
        let alts: Vec<(usize, &AltIR)> = self.rules().get(rule).unwrap().alts().iter().enumerate().collect();
        self.lookahead_alts(alts)
        
    }

    pub fn lookahead_alts<'a>(&'a self, alts: Vec<(usize, &'a AltIR)>) -> LookAheadNode<'a> {
        if alts.len() < 2 {
            return LookAheadNode::Terminal { alt: 0, continue_from: 0 }
        }

        let mut first: HashMap<&AtomIR, HashSet<usize>> = HashMap::new();
        for (index, alt) in &alts {
            let set = self.nth(alt, 0);
            for (atom, depth) in set {
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
            let alts = available_alts.iter().map(|index| alts.get(*index).unwrap().clone()).collect();
            out.insert(atom, self.lookahead_alts(alts));
        }

        LookAheadNode::Continues(LookAhead { tree: out })
    }
}