use std::{collections::BTreeSet, marker::PhantomData};

use serde::{Deserialize, Serialize};

use crate::{antlr::ast::{ANTLRAst, Alt, Atom, EBNFSuffix, Element, Rule, TokenRule}, codegen::{intermediate::rule::{RuleIR, TokenRuleIR}, symbols::SymbolTable}};

pub mod rule;
pub mod element;
pub mod alt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntlrIR {
    pub(super) rules: Vec<RuleIR>,
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
            symbol_table
        }
    }

    pub fn rules(&self) -> &Vec<RuleIR> {
        &self.rules
    }
    
    pub fn token_rules(&self) -> &Vec<TokenRuleIR> {
        &self.token_rules
    }

    pub fn symbols(&self) -> &SymbolTable {
        &self.symbol_table
    }

   
}