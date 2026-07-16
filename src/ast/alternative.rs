use std::{collections::HashSet, marker::PhantomData};

use serde::{Deserialize, Serialize};

use crate::{analysis::{AnalysisErr, SymbolTable}, ast::rules::{Atom, Element}, codegen::{ATNFragment, StateRef}};


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Alt {
    label: Option<String>,
    options: PhantomData<()>,
    elements: Vec<Element>,
    channel: Option<String>
}

impl Alt {
    pub fn new(label: Option<String>, elements: Vec<Element>, channel: Option<String>) -> Alt {
        Alt {
            label,
            elements,
            options: PhantomData,
            channel,
        }
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn nth(&self, n: usize) -> HashSet<&Element> {
        let mut set: HashSet<&Element> = HashSet::new();
        
        for element in &self.elements {
            let mut number = 0;
            match element {
                Element::Atom { .. } => {
                    
                },

                Element::Block { .. } => {

                },

                Element::Set { .. } => {

                }
            }
        };

        todo!()
    }

    pub fn codegen(&self, table: &SymbolTable) -> Result<ATNFragment, AnalysisErr> {
        let mut fragment = ATNFragment::new(); // TODO this is wrong
        for element in &self.elements {
            fragment.append_fragment(StateRef(0), element.codegen(table)?);
        }

        Ok(fragment)
    }

    pub fn symbols(&self, table: &mut SymbolTable) -> Result<(), AnalysisErr> {
        for element in &self.elements {
            element.symbols(table)?
        }

        Ok(())
    }
}

// AltList is a necessary struct because it can represent an anonymous alt list inside a block
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
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

    pub fn nth(&self, n: usize) -> HashSet<&Element> {
        let mut set = HashSet::new();
        for alt in &self.alts {
            set.extend(alt.nth(n));
        }

        set
    }

    pub fn symbols(&self, table: &mut SymbolTable) -> Result<(), AnalysisErr> {
        for alt in &self.alts {
            alt.symbols(table)?
        }

        Ok(())
    }

    pub fn codegen(&self, table: &SymbolTable) -> Result<ATNFragment, AnalysisErr> {
        let mut ands = true;
        let mut ors = false;

        for alt in &self.alts {
            ands = ands && alt.label().is_some();
            ors = ors || alt.label().is_some();
        }
        
        if ands != ors {
            return Err(AnalysisErr::AltLabels)
        };

        let mut fragment = ATNFragment::new();

        for alt in &self.alts {
            fragment.append_fragment(StateRef(0), alt.codegen(table)?);
        }

        Ok(fragment)
    }
}