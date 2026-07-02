use std::marker::PhantomData;

use crate::antlr::{ANTLRTokenType, Parser, ParserErr, parse::{alternative::Alt, ebnf::EBNFSuffix, rules::EBNFSuffix::{Star, StarOptional}}};

pub struct GrammarSpec {
    rule: Vec<Rule>
}

pub struct Rule {
    // modifiers: PhantomData<()>,
    // actions: PhantomData<()>,
    // return_val: PhantomData<()>,
    // throws_val: PhantomData<()>,
    // throws_spec: PhantomData<()>,
    // locals: PhantomData<()>,
    // prequel: PhantomData<()>,

    name: String,
    alts: Vec<Alt>
}

impl Rule {
    pub fn new(name: String, alts: Vec<Alt>) -> Rule {
        Rule {
            name,
            alts
        }
    }
}


pub enum Element {
    Atom {
        atom: Atom,
        suffix: Option<EBNFSuffix>
    },
    EBNF(EBNF)
}



