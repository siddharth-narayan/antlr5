use std::{collections::BTreeSet, marker::PhantomData};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ANTLRAst {
    rules: Vec<Rule>,
    token_rules: Vec<TokenRule>
}

impl ANTLRAst {
    pub fn new(rules: Vec<Rule>, token_rules: Vec<TokenRule>) -> ANTLRAst {
        ANTLRAst { rules, token_rules }
    }

    pub fn rules(&self) -> &Vec<Rule> {
        &self.rules
    }

    pub fn token_rules(&self) -> &Vec<TokenRule> {
        &self.token_rules
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRule {
    is_fragment: bool,
    name: String,
    alt_list: AltList
}

impl TokenRule {
    pub fn new(is_fragment: bool, name: String, alt_list: AltList) -> TokenRule {
        TokenRule {
            is_fragment,
            name,
            alt_list
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alts(&self) -> &Vec<Alt> {
        &self.alt_list.alts()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    // modifiers: PhantomData<()>,
    // actions: PhantomData<()>,
    // return_val: PhantomData<()>,
    // throws_val: PhantomData<()>,
    // throws_spec: PhantomData<()>,
    // locals: PhantomData<()>,
    // prequel: PhantomData<()>,

    name: String,
    alt_list: AltList
}

impl Rule {
    pub fn new(name: String, alt_list: AltList) -> Rule {
        Rule {
            name,
            alt_list
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alt_list(&self) -> &AltList {
        &self.alt_list
    }
    
    pub fn alts(&self) -> &Vec<Alt> {
        &self.alt_list.alts()
    }
}

// Should Element really have PartialEq/Eq derived?
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub enum Element {
    Atom {
        atom: Atom,
        suffix: Option<EBNFSuffix>
    },
    Block {
        block: Block,
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

#[derive(Debug, Serialize, Deserialize, Eq, Clone, PartialEq, Hash)]
pub struct Block(pub AltList);

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum Atom {
    StringLit(String),
    ID(String)
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct Alt {
    label: Option<String>,
    options: PhantomData<()>,
    elements: Vec<Element>,
    channel: Option<String>
}

impl Alt {
    pub fn new(label: Option<String>, elements: Vec<Element>, channel: Option<String>) -> Alt {
        Alt {
            label,
            elements,
            options: PhantomData,
            channel,
        }
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }
}

// AltList is a necessary struct because it can represent an anonymous alt list inside a block
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub struct AltList {
    optional: bool,
    alts: Vec<Alt>
}

impl AltList {
    pub fn new(optional: bool, alts: Vec<Alt>) -> AltList {
        AltList {
            optional,
            alts
        }
    }

    pub fn optional(&self) -> bool {
        self.optional
    }

    pub fn alts(&self) -> &Vec<Alt> {
        &self.alts
    }
}


#[derive(Clone, Copy, Debug, Serialize, Eq, PartialEq, Deserialize, Hash)]
pub enum EBNFSuffix {
    Optional,
    Star,
    // StarOptional, just star
    Plus,
    // PlusOptional, just star
}