use std::marker::PhantomData;

use crate::antlr::parse::rules::Element;

#[derive(Debug)]
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

#[derive(Debug)]
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