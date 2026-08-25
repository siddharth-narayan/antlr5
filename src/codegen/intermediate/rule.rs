use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    antlr::ast::{Rule, TokenRule},
    codegen::{intermediate::alt::AltIR, symbols::SymbolTable},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleIR {
    // modifiers: PhantomData<()>,
    // actions: PhantomData<()>,
    // return_val: PhantomData<()>,
    // throws_val: PhantomData<()>,
    // throws_spec: PhantomData<()>,
    // locals: PhantomData<()>,
    // prequel: PhantomData<()>,
    name: String,
    optional: bool,
    alts: Vec<Arc<AltIR>>,
}

impl RuleIR {
    pub fn new(rule: &Rule, table: &SymbolTable) -> Result<RuleIR, String> {
        let name = rule.name().clone();
        let optional = rule.alt_list().optional();
        let mut alts = Vec::new();

        if rule.alt_list().alts().len() == 0 {
            return Err("There are no nonempty alts".to_string());
        };

        for alt in rule.alts() {
            alts.push(Arc::new(AltIR::new(alt, table)?));
        }

        return Ok(RuleIR {
            name: name,
            optional,
            alts,
        });
    }

    pub fn new_tokenrule(rule: &TokenRule, table: &SymbolTable) -> Result<RuleIR, String> {
        let name = rule.name().clone();
        let optional = rule.alt_list().optional();
        let mut alts = Vec::new();

        if rule.alt_list().alts().len() == 0 {
            return Err("There are no nonempty alts".to_string());
        };

        for alt in rule.alts() {
            alts.push(Arc::new(AltIR::new(alt, table)?));
        }

        return Ok(RuleIR {
            name: name,
            optional,
            alts,
        });
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn alts(&self) -> &Vec<Arc<AltIR>> {
        &self.alts
    }
}