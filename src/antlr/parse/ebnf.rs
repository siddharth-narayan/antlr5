use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EBNFSuffix {
    Optional,
    Star,
    StarOptional,
    Plus,
    PlusOptional,
}