use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{ast::rules::Element, codegen::{ATNFragment, AnalysisErr}};


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

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
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

    pub fn alts(&self) -> &Vec<Alt> {
        &self.alts
    }

    pub fn codegen(&self) -> Result<ATNFragment, AnalysisErr> {
        let mut ands = true;
        let mut ors = false;

        for alt in &self.alts {
            ands = ands && alt.label().is_some();
            ors = ors || alt.label().is_some();
        }
        
        if ands != ors {
            return Err(AnalysisErr::AltLabels)
        };

        todo!()
    }
}