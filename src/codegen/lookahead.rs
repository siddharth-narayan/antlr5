use std::collections::{HashMap, HashSet};

use crate::codegen::{intermediate::{AltIR, AntlrIR, AtomIR, ElementIR, RuleIR}, symbols::SymbolTable};

pub struct First(HashSet<ElementIR>);

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
    pub fn new(rule: usize, ir: AntlrIR) -> LookAheadNode {
        let rule = ir.rules().get(rule).expect("bioajoij");

        if rule.alts().len() < 2 {
            return LookAheadNode::Terminal { alt: 0, continue_from: 0 }
        }


    }
}