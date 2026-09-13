use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    antlr::ast::{Block, Rule, TokenRule}, codegen::{intermediate::alt::AltIR, symbols::SymbolTable}, util::{Arena, HashArc},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleIR {
    is_block: bool,
    // modifiers: PhantomData<()>,
    // actions: PhantomData<()>,
    // return_val: PhantomData<()>,
    // throws_val: PhantomData<()>,
    // throws_spec: PhantomData<()>,
    // locals: PhantomData<()>,
    // prequel: PhantomData<()>,
    name: Option<String>,
    optional: bool,
    alts: Vec<HashArc<AltIR>>,
}

impl RuleIR {
    pub fn new(rule: &Rule, table: &SymbolTable, rules: &mut Arena<RuleIR>) -> Result<RuleIR, String> {
        let name = rule.name().clone();
        let optional = rule.alt_list().optional();
        let mut alts = Vec::new();

        if rule.alt_list().alts().len() == 0 {
            return Err("There are no nonempty alts".to_string());
        };

        for (alt_index, alt) in rule.alts().iter().enumerate() {
            let alt = AltIR::new(alt, alt_index, Some(table.get_rule_id(&name).expect("No rule found")), table, rules)?;
            alts.push(HashArc::new(alt));
        }

        return Ok(RuleIR {
            is_block: false,
            name: Some(name),
            optional,
            alts,
        });
    }

    pub fn new_tokenrule(rule: &TokenRule, table: &SymbolTable, rules: &mut Arena<RuleIR>) -> Result<RuleIR, String> {
        let name = rule.name().clone();
        let optional = rule.alt_list().optional();
        let mut alts = Vec::new();

        if rule.alt_list().alts().len() == 0 {
            return Err("There are no nonempty alts".to_string());
        };

        for (alt_index, alt) in rule.alts().iter().enumerate() {
            alts.push(HashArc::new(AltIR::new(alt, alt_index, Some(table.get_token_id(&name).expect("No rule found")), table, rules)?));
        }

        return Ok(RuleIR {
            is_block: false,
            name: Some(name),
            optional,
            alts,
        });
    }

    pub fn from_block(block: &Block, table: &SymbolTable, rules: &mut Arena<RuleIR>) -> Result<RuleIR, String> {
        let name = None;
        let optional = block.0.optional();
        let mut alts = Vec::new();

        if block.0.alts().len() == 0 {
            return Err("There are no nonempty alts".to_string());
        };

        for (alt_index, alt) in block.0.alts().iter().enumerate() {
            alts.push(HashArc::new(AltIR::new(alt, alt_index, None, table, rules)?));
        }

        return Ok(RuleIR {
            is_block: true,
            name: name,
            optional,
            alts,
        });
    }
    
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn alts(&self) -> &Vec<HashArc<AltIR>> {
        &self.alts
    }

    pub fn alts_mut(&mut self) -> &mut Vec<HashArc<AltIR>> {
        &mut self.alts
    }
}