use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{analysis::SymbolTable, ast::{ANTLRAst, EBNFSuffix}};

struct AntlrIR {
    rules: Vec<RuleIR>,
    token_rules: Vec<TokenRuleIR>,

    symbol_table: SymbolTable
}

impl AntlrIR {
    pub fn new(ast: ANTLRAst) -> AntlrIR {
        let mut symbol_table = SymbolTable::new();
        let rules = Vec::new();
        let token_rules = Vec::new();
        
        for rule in ast.rules() {
            symbol_table.insert_rule(rule.name().clone());
        }

        for rule in ast.token_rules() {
            symbol_table.insert_token_rule(rule.name().clone());
        }

        AntlrIR {
            rules,
            token_rules,
            symbol_table
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRuleIR {
    is_fragment: bool,
    name: String,
    is_optional: bool,
    alts: Vec<AltIR>
}

impl TokenRuleIR {
    pub fn new(is_fragment: bool, name: String, alt_list: AltListIR) -> TokenRuleIR {
        TokenRuleIR {
            is_fragment,
            name,
            alts
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alts(&self) -> &Vec<AltIR> {
        &self.alts
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleIR {
    // modifiers: PhantomData<()>,
    // actions: PhantomData<()>,
    // return_val: PhantomData<()>,
    // throws_val: PhantomData<()>,
    // throws_spec: PhantomData<()>,
    // locals: PhantomData<()>,
    // prequel: PhantomData<()>,

    name: String,
    is_optional: bool,
    alts: Vec<AltIR>
}

impl RuleIR {
    pub fn from(rule: Rule) -> RuleIR {
        RuleIR {
            name: rule.name().clone(),
            alt_list: AltListIR::new(rule.alt_list().clone())
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alts(&self) -> &Vec<AltIR> {
        &self.alt_list.alts()
    }


}


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AltIR {
    label: Option<String>,
    options: PhantomData<()>,
    elements: Vec<ElementIR>,
    channel: Option<String>
}

impl AltIR {
    pub fn new(label: Option<String>, elements: Vec<ElementIR>, channel: Option<String>) -> AltIR {
        AltIR {
            label,
            elements,
            options: PhantomData,
            channel,
        }
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn elements(&self) -> &Vec<ElementIR> {
        &self.elements
    }
}

// Should Element really have PartialEq/Eq derived?
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ElementIR {
    Atom {
        atom: usize,
        suffix: Option<EBNFSuffix>
    },
    Block {
        block: BlockIR,
        suffix: Option<EBNFSuffix>
    },
    // EBNF(EBNF)
}

impl ElementIR {
    pub fn new(e: Element) {
        e
    }

    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            ElementIR::Atom { suffix, .. } |
            ElementIR::Block { suffix, .. } => *suffix
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct BlockIR(Vec<AltIR>);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum AtomIR {
    TokenID(usize),
    RuleID(usize)
}