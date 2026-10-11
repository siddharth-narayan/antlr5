use crate::antlr::ast::EBNFSuffix;

// Represents the number of times a rule/token can repeat itself
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ordinal {
    pub is_optional: bool,
    pub inner: ConcreteOrdinal
}

impl Ordinal {
    pub fn from_suffix(suffix: Option<EBNFSuffix>) -> Ordinal {
        match suffix {
            Some(s) => {
                match s {
                    EBNFSuffix::Optional => Ordinal { is_optional: true, inner: ConcreteOrdinal::Finite(1) },
                    EBNFSuffix::Star => Ordinal { is_optional: true, inner: ConcreteOrdinal::Infinite { offset: 0, multiple: 0} },
                    EBNFSuffix::Plus => Ordinal { is_optional: false, inner: ConcreteOrdinal::Infinite { offset: 0, multiple: 0} }
                }
            },

            None => Ordinal { is_optional: false, inner: ConcreteOrdinal::Finite(1) }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ConcreteOrdinal {
    Finite(usize),
    Infinite {
        offset: usize,
        multiple: usize,
    }
}