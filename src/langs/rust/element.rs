use std::sync::Arc;

use crate::{antlr::ast::EBNFSuffix, codegen::intermediate::{AntlrIR, element::ElementIR}, util::capitalize};

pub fn element_name(ir: Arc<AntlrIR>, element: &ElementIR) -> Option<String> {
    match element {
        ElementIR::RuleAtom { id, suffix } => {
            Some(ir.rules().get(*id)?.name().clone())
        },

        ElementIR::TokenAtom { id, suffix } => {
            let name = ir.symbols().get_token_name(*id)?;
            Some(name)
        },

        ElementIR::Set { set, suffix, inverted } => {
           None
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
            let prefix = match element.suffix() {
                None => "Box<",
                Some(EBNFSuffix::Optional) => "Option<Box<",
                Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => "Vec<"
            };

            let suffix: String = match element.suffix() {
                Some(EBNFSuffix::Optional) => ">>".into(),
                None | Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => ">".into()
            };

            Some(format!("{}: {}{}{}", name_indexed, prefix, capitalize(name), suffix))
        },

        ElementIR::TokenAtom { .. } => {
            let prefix = match element.suffix() {
                None => "",
                Some(EBNFSuffix::Optional) => "Option<",
                Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => "Vec<"
            };

            let suffix = match element.suffix() {
                None => "",
                Some(EBNFSuffix::Optional) | Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => ">"
            };

            Some(format!("{}: {}Token{}", name_indexed, prefix, suffix))
        },
        _ => None
    }
}