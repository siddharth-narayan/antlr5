use std::collections::HashMap;

use crate::antlr::{ANTLRAst, rules::Rule};


pub enum Type {
    Token,
    Rule,
}

pub struct SymbolTable {
    table: HashMap<String, Type>
}

impl SymbolTable {
    pub fn build(tree: ANTLRAst) -> SymbolTable {
        let table = HashMap::new();
        SymbolTable { table }
    }

    pub fn visit_rule(table: &mut HashMap<String, Type>, rule: &Rule) {
        rule.
    }
}