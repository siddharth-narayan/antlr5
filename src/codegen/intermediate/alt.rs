use std::{marker::PhantomData, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{antlr::ast::{Alt, Atom, Element}, codegen::{intermediate::element::ElementIR, symbols::SymbolTable}};


#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AltIR {
    label: Option<String>,
    options: PhantomData<()>,
    elements: Vec<ElementIR>,
    channel: Option<String>,
}

impl AltIR {
    pub fn new(alt: &Alt, table: &SymbolTable) -> Result<AltIR, &'static str> {
        let label = alt.label().cloned();
        let channel = alt.channel().cloned();
        let mut elements = Vec::new();

        for (element_index, element) in alt.elements().iter().enumerate() {
            let element = match element {
                Element::Atom { atom, suffix } => {
                    match atom {
                        Atom::ID(n) => {
                            if let Some(id) = table.get_rule_id(&n) {
                                ElementIR::RuleAtom { id, suffix: *suffix}
                            } else if let Some(id) = table.get_token_id(&n) {
                                ElementIR::TokenAtom { id, suffix: *suffix}
                            } else {
                                return Err("No rule id found");
                            }
                        },
                        Atom::StringLit(n) => {
                            ElementIR::TokenAtom { id: table.get_strlit_id(&n).expect("Strlit's should all be processed"), suffix: *suffix }
                        }
                    }
                },
                Element::Block { block, suffix } => {
                    let _optional = block.0.optional(); // TODO use this for the suffix
                    let mut alts = Vec::new();
                    
                    for alt in block.0.alts() {
                        alts.push(Arc::new(AltIR::new(alt, table)?));
                    };

                    ElementIR::Block { block: alts, suffix: *suffix }
                },
                Element::Set { inverted: _, set: _, suffix: _ } => {
                    return Err("Parser rules cannot contain lexer sets")
                }

            };

            elements.push(element);
        };

        Ok(AltIR { label, options: PhantomData, elements, channel })
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn elements(&self) -> &Vec<ElementIR> {
        &self.elements
    }
}