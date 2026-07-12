use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum EBNFSuffix {
    Optional,
    Star,
    // StarOptional, just star
    Plus,
    // PlusOptional, just star
}