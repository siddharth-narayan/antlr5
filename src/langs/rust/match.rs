use std::{collections::HashMap, sync::Arc};

use rapidhash::fast::RandomState;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::MatchNode, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, langs::rust::element::{element_decl, element_name, element_name_indexed}, util::{HashArc, capitalize}};

pub fn match_node(ir: Arc<AntlrIR>, node: &MatchNode) -> String {
    match node {
        MatchNode::Peek { peek, fallback } => match_peek(ir.clone(), peek, fallback.as_ref()),
        MatchNode::Element { alt, element_idx, element, next } => match_element(ir, alt.clone(), element, *element_idx, next),
        MatchNode::Finish { alt } => match_finish(ir, alt.clone()),
    }
}

pub fn match_peek(ir: Arc<AntlrIR>, peek: &HashMap<usize, MatchNode, RandomState>, fallback: Option<&Box<MatchNode>>) -> String {
    let cases = peek.iter().map(|(key, value)| {
        match_case(ir.clone(), *key, value)
    }).collect::<Vec<_>>().join("\n");

    format!(
        "match self.peek().map(|t| t.token_type) {{
            {}

            None => return Err(ANTLRError::EOF),
            _ => return Err(ANTLRError::NoViableAlt),
        }}
        ",

        cases
    )
}

pub fn match_case(ir: Arc<AntlrIR>, key: usize, value: &MatchNode) -> String {
    let initializers = if let MatchNode::Element { alt, .. } = value {
        format!("{}", alt.elements().iter().enumerate().filter_map(|(e_idx, e)| match_element_initializer(ir.clone(), e, e_idx)).collect::<Vec<_>>().join("\n"))       
    } else {
        String::new()
    };

    format!(
        "
            Some({}) => {{
                {}
                {}
            }},
        ",

        key,
        initializers,
        match_node(ir.clone(), &value)
    )
}

pub fn match_element_initializer(ir: Arc<AntlrIR>, element: &ElementIR, element_idx: usize) -> Option<String> {
    match element.suffix() {
        None => None,
        Some(EBNFSuffix::Optional) => Some(format!("let {} = None;", element_decl(ir.clone(), element, element_idx)?)),
        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => Some(format!("let mut {} = Vec::new();", element_decl(ir.clone(), element, element_idx)?))
    }
}

pub fn match_element(ir: Arc<AntlrIR>, alt: HashArc<AltIR>, element: &ElementIR, element_idx: usize, next: &MatchNode) -> String {
    let name = element_name(ir.clone(), element);
    let name_indexed = element_name_indexed(ir.clone(), element, element_idx);

    let element = match element {
        ElementIR::RuleAtom { id, suffix } => {
            match suffix {
                None => format!("let {} = Box::new(self.{}()?);", name_indexed.unwrap(), name.unwrap()),
                Some(EBNFSuffix::Optional) => format!("let {} = self.{}().ok().map(Box::new);", name_indexed.unwrap(), name.unwrap()),
                Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!("while let Ok(__item) = self.{}() {{ {}.push(__item) }}", name.unwrap(), name_indexed.unwrap())
            }
        },
        ElementIR::TokenAtom { id, suffix } => {
            match ir.symbols().get_token_name(*id) {
                Some(_) => {
                    match suffix {
                        None => format!("let {} = self.match_token({})?.clone();", name_indexed.unwrap(), id),
                        Some(EBNFSuffix::Optional) => format!("let {} = self.match_token({}).ok();", name_indexed.unwrap(), id),
                        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!("while let Ok(x) = self.match_token({}) {{ {}.push(x) }}", id, name_indexed.unwrap())
                    }
                    
                },
                None => {
                    match suffix {
                        None => format!("let _ = self.match_token({})?;", id),
                        Some(EBNFSuffix::Optional) => format!("let _ = self.match_token({});", id),
                        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!("while let Ok(_) = self.match_token({}) {{}}", id)
                    }
                    
                }
            }
        },
        ElementIR::Set { set, suffix } => {
            "// Set placeholder\n".into()
        }
    };

    format!("{}\n{}", element, match_node(ir, &next))
}

pub fn match_finish(ir: Arc<AntlrIR>, alt: HashArc<AltIR>) -> String {
  let parent_rule = ir.get_rule(alt.parent()).unwrap();

    if parent_rule.alts().len() > 1 {
        let elements: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_name_indexed(ir.clone(), e, e_idx)).collect();

        format!(
            "{}::{} {{
                {}
            }}
            ", capitalize(parent_rule.name().clone()), alt.label().cloned().unwrap_or(format!("Alt{}", alt.index())), elements.join(",\n")
        )
    } else {
        let elements: Vec<_> = alt.elements().iter().enumerate().filter_map(
            |(e_idx, e)| element_name_indexed(ir.clone(), e, e_idx) ).collect();

        format!(
            "{}::new (
                {}
            )
            ", capitalize(parent_rule.name().clone()), elements.join(",\n")
        )
    }
}