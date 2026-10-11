use std::{collections::{BTreeSet}, sync::Arc};

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::numbers::Ordinal, intermediate::alt::AltIR}, util::HashArc};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ElementIR {
    RuleAtom {
        id: usize,
        size: Ordinal
    },
    TokenAtom {
        id: usize,
        size: Ordinal
    },
    Set {
        inverted: bool,
        set: BTreeSet<usize>,
        size: Ordinal
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

    pub fn size(&self) -> Ordinal {
        match self {
            ElementIR::RuleAtom { size, .. } |
            ElementIR::TokenAtom  { size, .. } |
            ElementIR::Set { size, .. } => *size
        }
    }
}

