
use std::{collections::{HashSet, VecDeque}, mem::MaybeUninit, ops::Deref, sync::Arc};

use rapidhash::fast::RandomState;
use serde::{Deserialize, Serialize};

use crate::{antlr::ast::ANTLRAst, codegen::{intermediate::{alt::AltIR, element::ElementIR, rule::RuleIR}, symbols::SymbolTable}, util::{Arena, HashArc, HashSetMap}};

pub mod rule;
pub mod element;
pub mod alt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntlrIR {
    ast: Arc<ANTLRAst>,
    rules: Vec<RuleIR>,
    token_rules: Vec<RuleIR>,

    symbol_table: SymbolTable,
}

impl AntlrIR {
    pub fn new(ast: ANTLRAst) -> AntlrIR {
        let ast = Arc::new(ast);
        let symbol_table = SymbolTable::new(&ast);

        let mut rules = Vec::with_capacity(ast.rules().len() + 100);
        for _ in 0..ast.rules().len() {
            rules.push(MaybeUninit::uninit())
        }

        for (rule_idx, rule) in ast.rules().iter().enumerate() {
            let rule = RuleIR::new(rule, &symbol_table, &mut rules).unwrap();
            rules[rule_idx] = MaybeUninit::new()
        }

        let mut rule_idx = 0;
        while let Some(mut rule) = rules.get_mut(rule_idx) {
            for alt in rule.assume_init().alts_mut() {
                let x = Arc::make_mut(alt);
            }
            rule_idx += 1;
        }

        let mut token_rules = Vec::new();
        for rule in ast.token_rules() {
            token_rules.push(RuleIR::new_tokenrule(rule, &symbol_table).unwrap())
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

    pub fn rules(&self) -> &Vec<RuleIR> {
        &self.rules
    }
    
    pub fn get_rule(&self, rule: usize) -> Option<RuleIR> {
        self.rules.get(rule).cloned()
    }

    pub fn get_rule_alt(&self, rule: usize, alt: usize) -> Option<HashArc<AltIR>> {
        self.rules.get(rule)?.alts().get(alt).cloned()
    }

    pub fn token_rules(&self) -> &Vec<RuleIR> {
        &self.token_rules
    }

    pub fn symbols(&self) -> &SymbolTable {
        &self.symbol_table
    }

    pub fn get_alt(&self, rule: usize, alt: usize) -> Option<HashArc<AltIR>> {
        self.rules.get(rule)?.alts().get(alt).cloned()
    }
}