use bimap::BiMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
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

// Lookahead and nth functions do not save their state for any alt, so likely could be a lot of performance gain saving the NTH set for each alt
pub fn nth(rules: &Vec<Arc<RuleIR>>, alt: Arc<AltIR>, n: usize) -> BiMap<ElementIR, usize> {
    let mut visited = HashSet::new();
    internal_nth(rules, alt, 0, n, 0, &mut visited)
}

// This function requires polonius to correctly understand control flow's lifetime situation
fn internal_nth(
    rules: &Vec<Arc<RuleIR>>,
    alt: Arc<AltIR>,
    starting_element: usize,

    n: usize,
    depth: usize,
    visited: &mut HashSet<usize>,
) -> BiMap<ElementIR, usize> {
    let mut pos = 0;
    let mut nth_atoms = BiMap::new();

    let elements = match alt.elements().get(starting_element..) {
        Some(e) => e,
        None => return nth_atoms
    };

    for (element_index, element) in elements.iter().enumerate() {
        match element.suffix() {
            Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Star) => {
                let mut new_visited = visited.clone();
                nth_atoms.extend(internal_nth(rules, alt.clone(), starting_element + element_index + 1, n - pos, depth + 1, &mut new_visited));
            },
            _ => ()
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

        pos += 1;
    }

    nth_atoms
}
