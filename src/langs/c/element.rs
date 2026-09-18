use std::{collections::HashMap, path::PathBuf, sync::Arc};

use rapidhash::fast::RandomState;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, langs::OutputFile, util::{HashArc, capitalize}};


pub fn element_name(ir: Arc<AntlrIR>, element: &ElementIR) -> Option<String> {
    match element {
        ElementIR::RuleAtom { id, suffix } => {
            Some(ir.rules().get(*id)?.name().clone())
        },

        ElementIR::TokenAtom { id, suffix } => {
            let name = ir.symbols().get_token_name(*id)?;
            Some(name)
        },

        ElementIR::Set { set, suffix } => {
           None
        }
    }
}

pub fn element_name_indexed(ir: Arc<AntlrIR>, element: &ElementIR, element_idx: usize) -> Option<String> {
    Some(format!("{}_{}", element_name(ir, element)?, element_idx))
}

pub fn element_type(ir: Arc<AntlrIR>, element: &ElementIR) -> String {
    let e_type = match element {
        ElementIR::RuleAtom { .. } => {
            format!("{}*", capitalize(element_name(ir.clone(), element).unwrap()))
        }
        
        _ => "Token*".into()
    };

    if let Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) = element.suffix() {
        "Array *".into()
    } else {
        e_type
    }
}

pub fn element_decl(ir: Arc<AntlrIR>, element: &ElementIR, element_idx: usize) -> Option<String> {
    let name = element_name(ir.clone(), element)?;
    
    Some(format!("{} {}_{}", element_type(ir.clone(), element), name, element_idx))
}
