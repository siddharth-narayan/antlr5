use serde::{Deserialize, Serialize};

use crate::{analysis::{AnalysisErr, SymbolTable}, ast::rules::{Rule, TokenRule}, codegen::{ATNFragment, StateRef}};

pub mod rules;
pub mod ebnf;
pub mod alternative;

#[derive(Debug, Serialize, Deserialize)]
pub struct ANTLRAst {
    rules: Vec<Rule>,
    token_rules: Vec<TokenRule>
}

impl ANTLRAst {
    pub fn new(rules: Vec<Rule>, token_rules: Vec<TokenRule>) -> ANTLRAst {
        ANTLRAst { rules, token_rules }
    }
}