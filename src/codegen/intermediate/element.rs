use std::{collections::BTreeSet, mem::discriminant, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{antlr::ast::EBNFSuffix, codegen::intermediate::alt::{AltIR, TokenAltIR}};

// Should Element really have PartialEq/Eq derived?
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ElementIR {
    Atom(AtomIR),
    Block {
        block: Vec<Arc<AltIR>>,
        suffix: Option<EBNFSuffix>
    },
    // EBNF(EBNF)
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum AtomIR {
    TokenID {
        id: usize,
        suffix: Option<EBNFSuffix>
    },

    RuleID {
        id: usize,
        suffix: Option<EBNFSuffix>
    }
}

impl AtomIR {
    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            AtomIR::RuleID { suffix, .. } | AtomIR::TokenID { suffix, .. } => suffix.clone()
        }
    }

    pub fn intersects(&self, other: &AtomIR) -> bool {
        if discriminant(self) != discriminant(other) || self.id() != other.id(){
            return false
        }

    }
}

impl ElementIR {
    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            ElementIR::Atom(a) => a.suffix(),
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

