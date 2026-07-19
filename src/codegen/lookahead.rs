use std::collections::HashMap;

use crate::codegen::intermediate::{AntlrIR, ElementIR};

pub struct LookAheadTree {
    tree: HashMap<ElementIR, LookAheadNode>
}

pub enum LookAheadNode {
    Continues(LookAheadTree),
    Terminal(usize) // The alt to pick
}

impl LookAheadTree {
    pub fn new(rule: usize, ir: AntlrIR) -> LookAheadTree {
        todo!()
    }
}