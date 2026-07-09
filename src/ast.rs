use serde::{Deserialize, Serialize};

use crate::{analysis::{AnalysisErr, SymbolTable}, ast::rules::Rule, codegen::{ATNFragment, StateRef}};

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