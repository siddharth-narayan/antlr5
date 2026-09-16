use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{
    antlr::ast::{Block, Rule, TokenRule}, codegen::{intermediate::alt::AltIR, symbols::SymbolTable}, util::{Arena, HashArc},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleIR {
    id: usize,

    is_block: bool,
    // modifiers: PhantomData<()>,
    // actions: PhantomData<()>,
    // return_val: PhantomData<()>,
    // throws_val: PhantomData<()>,
    // throws_spec: PhantomData<()>,
    // locals: PhantomData<()>,
    // prequel: PhantomData<()>,
    name: String,
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
            let alt = AltIR::new(alt, alt_index, table.get_rule_id(&name).expect("No rule found"), table, rules)?;
            alts.push(HashArc::new(alt));
        }

        return Ok(RuleIR {
            id: table.get_rule_id(&name).expect("No rule found"),
            is_block: false,
            name: name,
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
            alts.push(HashArc::new(AltIR::new(alt, alt_index, table.get_token_id(&name).expect("No rule found"), table, rules)?));
        }

        return Ok(RuleIR {
            id: table.get_token_id(&name).expect("No rule found"),
            is_block: false,
            name: name,
            optional,
            alts,
        });
    }

    pub fn from_block(parent_rule: usize, block: &Block, table: &SymbolTable, rules: &mut Arena<RuleIR>) -> Result<usize, String> {
        let optional = block.0.optional();
        let mut alts = Vec::new();
        let id = rules.push_index_landing_location(table.rule_count());

        unsafe { rules.mask(id); }

        if block.0.alts().len() == 0 {
            return Err("There are no nonempty alts".to_string());
        };

        for (alt_index, alt) in block.0.alts().iter().enumerate() {
            alts.push(HashArc::new(AltIR::new(alt, alt_index, id, table, rules)?));
        }


        let rule = RuleIR {
            id,
            is_block: true,
            name: format!("__anonymous_rule_{}", id),
            optional,
            alts,
        };

        rules.set(id, rule);

        Ok(id)
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn alts(&self) -> &Vec<HashArc<AltIR>> {
        &self.alts
    }

    pub fn alts_mut(&mut self) -> &mut Vec<HashArc<AltIR>> {
        &mut self.alts
    }
}