
use std::{collections::{HashSet, VecDeque}, sync::Arc};

use rapidhash::fast::RandomState;
use serde::{Deserialize, Serialize};

use crate::{antlr::ast::ANTLRAst, codegen::{intermediate::{alt::AltIR, element::ElementIR, rule::RuleIR}, symbols::SymbolTable}, util::HashSetMap};

pub mod rule;
pub mod element;
pub mod alt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntlrIR {
    ast: Arc<ANTLRAst>,
    rules: Vec<Arc<RuleIR>>,
    token_rules: Vec<Arc<RuleIR>>,

    symbol_table: SymbolTable,
}

impl AntlrIR {
    pub fn new(ast: ANTLRAst) -> AntlrIR {
        let ast = Arc::new(ast);
        let symbol_table = SymbolTable::new(&ast);

        let mut rules = Vec::new();
        for rule in ast.rules() {
            rules.push(Arc::new(RuleIR::new(rule, &symbol_table).unwrap()))
        }

        let mut token_rules = Vec::new();
        for rule in ast.token_rules() {
            token_rules.push(Arc::new(RuleIR::new_tokenrule(rule, &symbol_table).unwrap()))
        }
        
        AntlrIR {
            ast: ast,
            rules,
            token_rules,
            symbol_table,
        }
    }

    pub fn nth(&self, n: usize, rule: usize) -> Option<HashSet<ElementIR, RandomState>> {
        let mut nth_set = HashSet::default();

        for alt in self.get_rule(rule)?.alts() {
            let n = crate::nth(n, 0, (alt.clone(), 0), &mut VecDeque::new(), &mut HashSetMap::new(), &mut HashSet::default(), self.rules()).cloned().unwrap_or_default();
            nth_set.extend(n);
        }

        Some(nth_set)
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