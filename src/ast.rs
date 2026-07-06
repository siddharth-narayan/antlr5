use serde::{Deserialize, Serialize};

use crate::ast::rules::Rule;

pub mod rules;
pub mod ebnf;
pub mod alternative;

#[derive(Debug, Serialize, Deserialize)]
pub struct ANTLRAst {
    rules: Vec<Rule>
}

impl ANTLRAst {
    pub fn new(rules: Vec<Rule>) -> ANTLRAst {
        ANTLRAst { rules }
    }
}