use std::{collections::BTreeSet, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{antlr::ast::EBNFSuffix, codegen::intermediate::alt::{AltIR, TokenAltIR}};

// Should Element really have PartialEq/Eq derived?
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ElementIR {
    Atom {
        atom: AtomIR,
        suffix: Option<EBNFSuffix>
    },
    Block {
        block: Vec<Arc<AltIR>>,
        suffix: Option<EBNFSuffix>
    },
    // EBNF(EBNF)
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum AtomIR {
    TokenID(usize),
    RuleID(usize)
}

impl ElementIR {
    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            ElementIR::Atom { suffix, .. } |
            ElementIR::Block { suffix, .. } => *suffix
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum TokenElementIR {
    Atom {
        atom: usize,
        suffix: Option<EBNFSuffix>
    },
    Set {
        inverted: bool,
        set: BTreeSet<usize>,
        suffix: Option<EBNFSuffix>
    },
    Block {
        block: Vec<TokenAltIR>,
        suffix: Option<EBNFSuffix>
    },
    // EBNF(EBNF)
}

impl TokenElementIR {
    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            TokenElementIR::Atom { suffix, .. } |
            TokenElementIR::Block { suffix, .. } |
            TokenElementIR::Set { suffix, .. } => *suffix
        }
    }
}

