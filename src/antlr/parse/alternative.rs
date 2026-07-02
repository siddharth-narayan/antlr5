use std::marker::PhantomData;

use crate::antlr::parse::rules::Element;

pub enum Alt {
    Empty,
    Some {
        options: PhantomData<()>,
        elements: Vec<Element>,
    }
}
