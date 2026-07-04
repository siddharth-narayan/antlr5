use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::antlr::parse::rules::Element;

#[derive(Debug, Serialize, Deserialize)]
pub struct Alt {
    label: Option<String>,
    options: PhantomData<()>,
    elements: Vec<Element>,
}

impl Alt {
    pub fn new(label: Option<String>, elements: Vec<Element>) -> Alt {
        Alt {
            label,
            elements,
            options: PhantomData
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AltList {
    optional: bool,
    alts: Vec<Alt>
}

impl AltList {
    pub fn new(optional: bool, alts: Vec<Alt>) -> AltList {
        AltList {
            optional,
            alts
        }
    }
}