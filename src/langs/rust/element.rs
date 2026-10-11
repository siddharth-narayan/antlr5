use std::sync::Arc;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::numbers::ConcreteOrdinal, intermediate::{AntlrIR, element::ElementIR}}, util::capitalize};

pub fn element_name(ir: Arc<AntlrIR>, element: &ElementIR) -> Option<String> {
    match element {
        ElementIR::RuleAtom { id, size } => {
            Some(ir.rules().get(*id)?.name().clone())
        },

        ElementIR::TokenAtom { id, size } => {
            let name = ir.symbols().get_token_name(*id)?;
            Some(name)
        },

        ElementIR::Set { set, size, inverted } => {
           Some("SET".into())
        }
    }
}

pub fn element_name_indexed(ir: Arc<AntlrIR>, element: &ElementIR, element_idx: usize) -> Option<String> {
    Some(format!("{}_{}", element_name(ir, element)?, element_idx))
}

pub fn element_decl(ir: Arc<AntlrIR>, element: &ElementIR, element_idx: usize) -> Option<String> {
    let name = element_name(ir.clone(), element)?;
    let name_indexed = element_name_indexed(ir.clone(), element, element_idx)?;


    match element {
        ElementIR::RuleAtom { .. } => {
            let prefix = if let ConcreteOrdinal::Finite(_) = element.size().inner {
                if element.size().is_optional {
                    "std::option::Option<Box<"
                } else {
                    "Box<"
                }
            } else {
                "Vec<"
            };

            let suffix = if let ConcreteOrdinal::Finite(_) = element.size().inner {
                if element.size().is_optional {
                    ">>"
                } else {
                    ">"
                }
            } else {
                ">"
            };

            Some(format!("{}: {}{}{}", name_indexed, prefix, capitalize(name), suffix))
        },

        ElementIR::TokenAtom { .. } => {

            let prefix = if let ConcreteOrdinal::Finite(_) = element.size().inner {
                if element.size().is_optional {
                    "std::option::Option<"
                } else {
                    ""
                }
            } else {
                "Vec<"
            };


            let suffix = if let ConcreteOrdinal::Finite(_) = element.size().inner {
                if element.size().is_optional {
                    ">"
                } else {
                    ""
                }
            } else {
                ">"
            };

            Some(format!("{}: {}Token{}", name_indexed, prefix, suffix))
        },
        _ => None
    }
}