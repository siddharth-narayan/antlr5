use std::{collections::{BTreeSet}, sync::Arc};

use crate::{antlr::ast::EBNFSuffix, codegen::intermediate::alt::AltIR, util::HashArc};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ElementIR {
    RuleAtom {
        id: usize,
        suffix: Option<EBNFSuffix>
    },
    TokenAtom {
        id: usize,
        suffix: Option<EBNFSuffix>
    },
    Set {
        inverted: bool,
        set: BTreeSet<usize>,
        suffix: Option<EBNFSuffix>
    },
    // EBNF(EBNF)
}

impl ElementIR {
    pub fn id(&self) -> Option<usize> {
        match self {
            ElementIR::RuleAtom { id, .. } | ElementIR::TokenAtom { id, .. } => Some(*id),
            _ => None
        }
    }

    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            ElementIR::RuleAtom { suffix, .. } |
            ElementIR::TokenAtom  { suffix, .. } |
            ElementIR::Set { suffix, .. } => *suffix
        }
    }
}

