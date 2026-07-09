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
    rules: HashMap<String, usize>,
    token_rules: HashMap<String, usize>,

    // Tokens are different from tokens in that they're anonymous fragments of text
    tokens: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { rules: HashMap::new(), token_rules: HashMap::new(), tokens: HashMap::new() }
    }

    pub fn get_rule_id(&self, name: String) -> Option<usize> {
        self.rules.get(&name).cloned()
    }

    pub fn get_token_id(&self, name: String) -> Option<usize> {
        self.tokens.get(&name).cloned()
    }

    pub fn insert_rule(&mut self, name: String) -> Result<(), AnalysisErr> {
        if self.rules.contains_key(&name) {
            return Err(AnalysisErr::Redefinition { of: name })
        }

        self.rules.insert(name, self.rules.len());

        Ok(())
    }

    pub fn insert_token_rule(&mut self, name: String) -> Result<(), AnalysisErr> {
        if self.token_rules.contains_key(&name) {
            return Err(AnalysisErr::Redefinition { of: name })
        }

        self.token_rules.insert(name, self.token_rules.len());

        Ok(())
    }

    pub fn insert_token(&mut self, name: String) {
        if self.tokens.contains_key(&name) {
            return
        }
        self.tokens.insert(name, self.tokens.len());
    }
}