use std::{collections::{BTreeSet, HashMap, HashSet}, marker::PhantomData};

use serde::{Deserialize, Serialize};

use crate::ast::{ebnf::EBNFSuffix, rules::{Rule, TokenRule}};

#[derive(Debug, Serialize, Deserialize, Hash)]
pub struct RuleRef(usize);

struct AntlrIR {
    rules: Vec<RuleIR>,
    token_rules: Vec<TokenRuleIR>,

    rule_map: HashMap<String, usize>,
    token_map: HashMap<String, usize>,
    strlit_map: HashSet<String, usize>
}

impl AntlrIR {
    pub fn new(rules: Vec<Rule>, token_rules: Vec<TokenRule>) -> AntlrIR {
        let mut rule_map = HashMap::new();
        for (index, rule) in rules.iter().enumerate() {
            rule_map.insert(rule.name().clone(), index);
        }

        let mut token_map = HashMap::new();
        for (index, rule) in rules.iter().enumerate() {
            rule_map.insert(rule.name().clone(), index);
        }

        AntlrIR {
            rules,
            token_rules,
            rule_map,
            token_map
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRuleIR {
    is_fragment: bool,
    name: String,
    alt_list: AltListIR
}

impl TokenRuleIR {
    pub fn new(is_fragment: bool, name: String, alt_list: AltListIR) -> TokenRuleIR {
        TokenRuleIR {
            is_fragment,
            name,
            alt_list
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alts(&self) -> &Vec<AltIR> {
        &self.alt_list.alts()
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
    alt_list: AltListIR
}

impl RuleIR {
    pub fn new(name: String, alt_list: AltListIR) -> RuleIR {
        RuleIR {
            name,
            alt_list
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

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AltListIR {
    optional: bool,
    alts: Vec<AltIR>
}

impl AltListIR {
    pub fn new(optional: bool, alts: Vec<AltIR>) -> AltListIR {
        AltListIR {
            optional,
            alts
        }
    }

    pub fn alts(&self) -> &Vec<AltIR> {
        &self.alts
    }
}

// Should Element really have PartialEq/Eq derived?
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Element {
    Atom {
        atom: usize,
        suffix: Option<EBNFSuffix>
    },
    Block {
        block: BlockIR,
        suffix: Option<EBNFSuffix>
    },
    Set {
        inverted: bool,
        set: BTreeSet<usize>,
        suffix: Option<EBNFSuffix>
    }
    // EBNF(EBNF)
}

impl Element {
    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            Element::Atom { suffix, .. } |
            Element::Block { suffix, .. } |
            Element::Set { suffix, .. } => *suffix
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct BlockIR(pub AltListIR);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum AtomIR {
    StringLit(R),
    ID(RuleRef)
}