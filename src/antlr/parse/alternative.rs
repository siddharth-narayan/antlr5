use std::marker::PhantomData;

use crate::antlr::parse::rules::Element;

#[derive(Debug)]
pub enum Alt {
    Empty,
    Some {
        options: PhantomData<()>,
        elements: Vec<Element>,
    }
}


impl Alt {
    pub fn new(label: String, elements: Vec<Element>) -> Alt {
        if elements.len() == 0 {
            Alt::Empty
        } else {
            Alt::Some { options: PhantomData, elements }
        }
    }
}