
use std::{collections::HashMap, sync::Arc};

use bimap::BiMap;
use serde::{Deserialize, Serialize};

use crate::{antlr::ast::ANTLRAst, codegen::{RuleRef, analysis::{LookAhead, LookAheadNode}, intermediate::{alt::AltIR, element::AtomIR, rule::{RuleIR, TokenRuleIR}}, symbols::SymbolTable}};

pub mod rule;
pub mod element;
pub mod alt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntlrIR {
    rules: Vec<RuleIR>,
    token_rules: Vec<TokenRuleIR>,

    symbol_table: SymbolTable,
}

impl AntlrIR {
    pub fn new(ast: ANTLRAst) -> AntlrIR {
        let symbol_table = SymbolTable::new(ast);

        let rules = symbol_table.rules().iter().map(|r| RuleIR::new(r, &symbol_table).unwrap()).collect();
        let token_rules = symbol_table.token_rules().iter().map(|r| TokenRuleIR::new(r, &symbol_table).unwrap()).collect();
        
        AntlrIR {
            rules,
            token_rules,
            symbol_table,
        }
    }

    pub fn rules(&self) -> &Vec<RuleIR> {
        &self.rules
    }
    
    pub fn get_rule(&self, rule: usize) -> Option<&RuleIR> {
        self.rules.get(rule)
    }

    pub fn token_rules(&self) -> &Vec<TokenRuleIR> {
        &self.token_rules
    }

    pub fn symbols(&self) -> &SymbolTable {
        &self.symbol_table
    }

    pub fn get_alt(&self, rule: usize, alt: usize) -> Option<Arc<AltIR>> {
        self.rules.get(rule)?.alts().get(alt).cloned()
    }
}