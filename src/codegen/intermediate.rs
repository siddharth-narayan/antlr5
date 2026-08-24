
use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{antlr::ast::ANTLRAst, codegen::{RuleRef, analysis::{Choice, MatchNode}, intermediate::{alt::AltIR, rule::RuleIR}, symbols::SymbolTable}};

pub mod rule;
pub mod element;
pub mod alt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntlrIR {
    rules: Vec<Arc<RuleIR>>,
    token_rules: Vec<Arc<RuleIR>>,

    symbol_table: SymbolTable,
}

impl AntlrIR {
    pub fn new(ast: ANTLRAst) -> AntlrIR {
        let symbol_table = SymbolTable::new(ast);

        let mut rules = Vec::new();
        for rule in symbol_table.rules() {
            rules.push(Arc::new(RuleIR::new(rule, &symbol_table).unwrap()))
        }

        let mut token_rules = Vec::new();
        for rule in symbol_table.token_rules() {
            token_rules.push(Arc::new(RuleIR::new_tokenrule(rule, &symbol_table).unwrap()))
        }
        
        AntlrIR {
            rules,
            token_rules,
            symbol_table,
        }
    }

    pub fn rules(&self) -> &Vec<Arc<RuleIR>> {
        &self.rules
    }
    
    pub fn get_rule(&self, rule: usize) -> Option<Arc<RuleIR>> {
        self.rules.get(rule).cloned()
    }

    pub fn get_rule_alt(&self, rule: usize, alt: usize) -> Option<Arc<AltIR>> {
        self.rules.get(rule)?.alts().get(alt).cloned()
    }

    pub fn token_rules(&self) -> &Vec<Arc<RuleIR>> {
        &self.token_rules
    }

    pub fn symbols(&self) -> &SymbolTable {
        &self.symbol_table
    }

    pub fn get_alt(&self, rule: usize, alt: usize) -> Option<Arc<AltIR>> {
        self.rules.get(rule)?.alts().get(alt).cloned()
    }
}