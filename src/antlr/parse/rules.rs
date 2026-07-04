use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::antlr::{ANTLRTokenType, Parser, ParserErr, parse::{alternative::{Alt, AltList}, ebnf::EBNFSuffix, rules::EBNFSuffix::{Star, StarOptional}}};

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
    alts: AltList
}

impl Rule {
    pub fn new(name: String, alts: AltList) -> Rule {
        Rule {
            name,
            alts
        }
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

