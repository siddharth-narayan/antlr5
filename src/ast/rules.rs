use serde::{Deserialize, Serialize};

use crate::ast::{alternative::{Alt, AltList}, ebnf::EBNFSuffix};

pub struct GrammarSpec {
    rule: Vec<Rule>
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

    pub fn alts(&self) -> &Vec<Alt> {
        &self.alt_list.alts()
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Element {
    Atom {
        atom: Atom,
        suffix: Option<EBNFSuffix>
    },
    Block {
        block: Block,
        suffix: Option<EBNFSuffix>
    }
    // EBNF(EBNF)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block(pub AltList);

#[derive(Debug, Serialize, Deserialize)]
pub enum Atom {
    StringLit(String),
    ID(String)
}

