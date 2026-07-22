use std::{collections::{HashMap, HashSet}, sync::Arc};

use crate::codegen::{intermediate::{AltIR, AntlrIR, AtomIR, ElementIR, RuleIR}, symbols::SymbolTable};

pub struct LookAhead {
    tree: HashMap<ElementIR, LookAheadNode>
}

pub enum LookAheadNode {
    Continues(LookAhead),
    Terminal {
        alt: usize, // The alt to pick
        continue_from: usize // The element that needs to next be matched
    }
}

impl LookAheadNode {
    pub fn new(rule: usize, ir: Arc<AntlrIR>) -> LookAheadNode {
        let rule = ir.rules().get(rule).expect("bioajoij");

        if rule.alts().len() < 2 {
            return LookAheadNode::Terminal { alt: 0, continue_from: 0 }
        }

        let mut intersection = HashSet::new();
        while !intersection.is_empty() {
            

            
        }
        



        let firsts: Vec<HashSet<&ElementIR>> = rule.alts().iter().map(|a| ir.nth(a, 0)).collect();

        let mut intersection: HashSet<&ElementIR> = HashSet::new();
        for (index, set) in firsts.iter().enumerate() {
            for compare in firsts.iter().skip(index + 1) {
                let x: HashSet<&ElementIR> = set.intersection(compare).map(|e| *e).collect();
                intersection = intersection.union(&x).map(|e| *e).collect();
            }
        }

        println!("intersection: {:#?}", intersection);
        
        todo!()
    }
}

enum LookAheadMatch {
    F
}
pub fn matches(e: ElementIR, with: ElementIR) {
    
}