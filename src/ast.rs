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

    pub fn codegen(&self, table: &SymbolTable) -> Result<ATNFragment, AnalysisErr> {
        let mut atn = ATNFragment::new();
        
        for rule in &self.rules {
            atn.append_fragment(StateRef(0), rule.codegen(table)?);
        }

        Ok(atn)
    }

    pub fn symbols(&self, table: &mut SymbolTable) -> Result<(), AnalysisErr> {
        for rule in &self.rules {
            rule.symbols(table)?
        }

        Ok(())
    }
}