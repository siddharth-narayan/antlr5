use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::{antlr::ast::EBNFSuffix, codegen::intermediate::alt::{AltIR}};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ElementIR {
    RuleAtom {
        id: usize,
        suffix: Option<EBNFSuffix>
    },
    TokenAtom {
        id: usize,
        suffix: Option<EBNFSuffix>
    },
    TokenSet {
        id: usize,
        suffix: Option<EBNFSuffix>
    },
    Block {
        block: Vec<Arc<AltIR>>,
        suffix: Option<EBNFSuffix>
    },
    // EBNF(EBNF)
}

impl ElementIR {
    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            ElementIR::RuleAtom { suffix, .. } |
            ElementIR::TokenAtom  { suffix, .. } |
            ElementIR::TokenSet { suffix, .. } |
            ElementIR::Block { suffix, .. } => *suffix
        }
    }
}