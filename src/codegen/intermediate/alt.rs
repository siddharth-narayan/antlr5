use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{marker::PhantomData, sync::Arc};

use crate::{
    antlr::ast::{Alt, Atom, Element},
    codegen::{intermediate::element::ElementIR, symbols::SymbolTable},
};

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AltIR {
    index: usize,
    parent_rule: Option<usize>,

    label: Option<String>,
    options: PhantomData<()>,
    elements: Vec<ElementIR>,
    channel: Option<String>,
}

impl AltIR {
    pub fn new(
        alt: &Alt,
        index: usize,
        parent_rule: Option<usize>,
        table: &SymbolTable,
    ) -> Result<AltIR, String> {
        let label = alt.label().cloned();
        let channel = alt.channel().cloned();
        let mut elements = Vec::new();

        for (element_index, element) in alt.elements().iter().enumerate() {
            let element = match element {
                Element::Atom { atom, suffix } => match atom {
                    Atom::ID(n) => {
                        if let Some(id) = table.get_rule_id(&n) {
                            ElementIR::RuleAtom {
                                id,
                                suffix: *suffix,
                            }
                        } else if let Some(id) = table.get_token_id(&n) {
                            ElementIR::TokenAtom {
                                id,
                                suffix: *suffix,
                            }
                        } else {
                            return Err(format!("No rule found for id {}", n).to_string());
                        }
                    }
                    Atom::StringLit(n) => ElementIR::TokenAtom {
                        id: table
                            .get_strlit_id(&n)
                            .expect("Strlit's should all be processed"),
                        suffix: *suffix,
                    },
                },
                Element::Block { block, suffix } => {
                    let _optional = block.0.optional(); // TODO use this for the suffix
                    let mut alts = Vec::new();

                    for (alt_index, alt) in block.0.alts().iter().enumerate() {
                        alts.push(Arc::new(AltIR::new(alt, alt_index, None, table)?));
                    }

                    ElementIR::Block {
                        block: alts,
                        suffix: *suffix,
                    }
                }
                Element::Set {
                    inverted: _,
                    set,
                    suffix,
                } => ElementIR::TokenSet {
                    set: set.clone(),
                    suffix: *suffix,
                },
            };

            elements.push(element);
        }

        Ok(AltIR {
            parent_rule,
            index,
            label,
            options: PhantomData,
            elements,
            channel,
        })
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn elements(&self) -> &Vec<ElementIR> {
        &self.elements
    }
}

impl Debug for AltIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AltIR")
            .field("index", &self.index)
            .field("parent_rule", &self.parent_rule)
            // .field("label", &self.label)
            // .field("options", &self.options)
            // .field("elements", &self.elements)
            // .field("channel", &self.channel)
            .finish()
    }
}
