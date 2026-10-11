use rapidhash::fast::RandomState;
use tracing::{error, instrument};
use std::{
    char, collections::{HashMap, HashSet, VecDeque, hash_set::IntoIter}, hash::Hash, mem::discriminant, sync::Arc,
};
use std::fmt::Debug;
use crate::{
    antlr::ast::{EBNFSuffix, Element}, codegen::{
        RuleRef, analysis::r#match::MatchNode, intermediate::{
            AntlrIR, alt::{self, AltIR}, element::{self, ElementIR}, rule::RuleIR,
        },
    }, util::{HashArc, HashSetMap},
};



#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Peek {
    pub(super) map: HashMap<usize, PeekResult>, // The simple token we match on
    pub(super) fallback: Option<Box<MatchNode>>
}

impl Peek {
    pub fn new() -> Peek {
        Peek {
            map: HashMap::new(),
            fallback: None
        }
    }

    pub fn map(&self) -> &HashMap<usize, PeekResult> {
        &self.map
    }

    pub fn fallback(&self) -> Option<&Box<MatchNode>> {
        self.fallback.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PeekResult {
    pub(super) greedy: bool,
    pub(super) can_become_greedy: bool,

    pub(super) match_node: MatchNode
}

impl PeekResult {
    pub fn new(match_node: MatchNode) -> PeekResult {
        let can_become_greedy = if let MatchNode::Element { ref element, .. } = match_node {
            match element.suffix() {
                None => false,
                Some(EBNFSuffix::Optional) => false,
                Some(EBNFSuffix::Plus) => true,
                Some(EBNFSuffix::Star) => true
            }
        } else {
            false
        };

        PeekResult {
            greedy: false,
            can_become_greedy,
            match_node
        }
    }

    pub fn merge(&mut self, other: PeekResult, ir: Arc<AntlrIR>) {

    }

    pub fn match_node(&self) -> &MatchNode {
        &self.match_node
    }
}
