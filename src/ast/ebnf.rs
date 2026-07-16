use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Eq, PartialEq, Deserialize, Hash)]
pub enum EBNFSuffix {
    Optional,
    Star,
    // StarOptional, just star
    Plus,
    // PlusOptional, just star
}