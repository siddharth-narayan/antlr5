use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum AnalysisErr {
    Undefined {
        name: String
    },

    Redefinition {
        of: String
    },

    AltLabels
}

#[derive(Debug)]
pub struct SymbolTable {
    rule_map: HashMap<String, usize>,
    token_map: HashMap<String, usize>,
    strlit_map: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { rule_map: HashMap::new(), token_map: HashMap::new(), strlit_map: HashMap::new() }
    }

    pub fn get_rule_id(&self, name: String) -> Option<usize> {
        self.rule_map.get(&name).cloned()
    }

    pub fn get_token_id(&self, name: String) -> Option<usize> {
        self.strlit_map.get(&name).cloned()
    }

    pub fn insert_rule(&mut self, name: String) -> Result<(), AnalysisErr> {
        if self.rule_map.contains_key(&name) {
            return Err(AnalysisErr::Redefinition { of: name })
        }

        self.rule_map.insert(name, self.rule_map.len());

        Ok(())
    }

    pub fn insert_token_rule(&mut self, name: String) -> Result<(), AnalysisErr> {
        if self.token_map.contains_key(&name) {
            return Err(AnalysisErr::Redefinition { of: name })
        }

        self.token_map.insert(name, self.token_map.len());

        Ok(())
    }

    pub fn insert_token(&mut self, name: String) {
        if self.strlit_map.contains_key(&name) {
            return
        }
        self.strlit_map.insert(name, self.strlit_map.len());
    }
}