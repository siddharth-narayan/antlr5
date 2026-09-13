
use serde::{Deserialize, Serialize};

use crate::{antlr::ast::{ANTLRAst, Alt, Atom, Element, Rule, TokenRule}, util::BiMap};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolTable {
    // ast: ANTLRAst,

    rule_map: BiMap<String, usize>,
    token_map: BiMap<String, usize>,
    strlit_map: BiMap<String, usize>,
}

pub fn get_strlits(alt: &Alt) -> Vec<String> {
    let mut out  = Vec::new();
    for element in alt.elements() {
        match element {
            Element::Atom{ atom: Atom::StringLit(s), .. } => {
                out.push(s.clone());
            },
            Element::Block { block, .. } => {
                for alt in block.0.alts() {
                    out.extend(get_strlits(alt));
                }
            },
            _ => ()
        }
    }

    out
}

impl SymbolTable {
    pub fn new(ast: &ANTLRAst) -> SymbolTable {
        let mut table = SymbolTable { rule_map: BiMap::new(), token_map: BiMap::new(), strlit_map: BiMap::new() };

        // I don't want to clone everywhere :(
        for rule in ast.rules().clone() {

            for alt in rule.alts() {
                for strlit in get_strlits(alt) {
                    table.insert_strlit(strlit);
                }
            }
        }

        for rule in ast.token_rules().clone() {
            for alt in rule.alts() {
                for strlit in get_strlits(alt) {
                    table.insert_strlit(strlit);
                }
            }
        }
        
        table.insert_token_rule("EOF".into(), usize::MAX);
        table
    }

    pub fn get_rule_id(&self, name: &String) -> Option<usize> {
        self.rule_map.get(name).cloned()
    }
    
    pub fn get_rule_name(&self, id: usize) -> Option<String> {
        self.rule_map.get_inverse(&id).cloned()
    }

    pub fn get_token_id(&self, name: &String) -> Option<usize> {
        self.token_map.get(name).cloned()
    }
    
    pub fn get_token_name(&self, id: usize) -> Option<String> {
        self.token_map.get_inverse(&id).cloned()
    }
    
    pub fn get_strlit_id(&self, name: &String) -> Option<usize> {
        self.strlit_map.get(name).cloned()
    }

    pub fn get_strlit_name(&self, id: usize) -> Option<String> {
        self.strlit_map.get_inverse(&id).cloned()
    }

    pub fn insert_rule(&mut self, name: String, index: usize) -> Result<(), AnalysisErr> {
        if self.rule_map.contains(&name) {
            return Err(AnalysisErr::Redefinition { of: name })
        }

        self.rule_map.insert(name, index);

        Ok(())
    }

    pub fn insert_token_rule(&mut self, name: String, index: usize) -> Result<(), AnalysisErr> {
        if self.token_map.contains(&name) {
            return Err(AnalysisErr::Redefinition { of: name })
        }

        self.token_map.insert(name, index);

        Ok(())
    }

    pub fn insert_strlit(&mut self, name: String) {
        if self.strlit_map.contains(&name) {
            return
        }
        self.strlit_map.insert(name, self.token_map.len() + self.strlit_map.len());
    }
}