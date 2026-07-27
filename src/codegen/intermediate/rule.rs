use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{antlr::ast::{Rule, TokenRule}, codegen::{intermediate::alt::{AltIR, TokenAltIR}, symbols::SymbolTable}};

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
    alts: Vec<Arc<AltIR>>
}

impl RuleIR {
    pub fn new(rule: &Rule, table: &SymbolTable) -> Result<RuleIR, &'static str> {
        let name = rule.name().clone();
        let optional = rule.alt_list().optional();
        let mut alts = Vec::new();

        if rule.alt_list().alts().len() == 0 {
            return Err("There are no nonempty alts");
        };
        

        let id = table.get_rule_id(&name).expect("No ruleid");

        for alt in rule.alts() {
            alts.push(Arc::new(AltIR::new(id, alt, table)?));
        }



        return Ok(RuleIR {
            name: name,
            optional,
            alts
        })

    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alts(&self) -> &Vec<Arc<AltIR>> {
        &self.alts
    }


}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRuleIR {
    is_fragment: bool,
    name: String,
    optional: bool,
    alts: Vec<TokenAltIR>
}

impl TokenRuleIR {
    pub fn new(rule: &TokenRule, symbols: &SymbolTable) -> Result<TokenRuleIR, &'static str> {
        let _name = rule.name().clone();
        let is_fragment = rule.is_fragment();
        let optional = rule.alt_list().optional();
        let mut alts = Vec::new();

        if rule.alt_list().alts().len() == 0 {
            return Err("There are no nonempty alts");
        };
        

        for alt in rule.alts() {
            alts.push(TokenAltIR::new(alt, symbols)?);
        }

        return Ok(TokenRuleIR {
            is_fragment,
            name: rule.name().clone(),
            optional,
            alts: alts
        })
    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alts(&self) -> &Vec<TokenAltIR> {
        &self.alts
    }
}

