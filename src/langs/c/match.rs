use std::{collections::HashMap, path::PathBuf, sync::Arc};

use rapidhash::fast::RandomState;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, langs::{OutputFile, c::element::{element_decl, element_name, element_name_indexed}}, util::{HashArc, capitalize}};


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

        "Token* next_token = peek(parser);
        if (next_token == NULL) {{
            errno = ERANGE;
            return NULL;
        }}
        
        switch (next_token->token_type) {{
            {}
            default:
                exit(-1);
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
            case {}: {{
                {}
                {}
                break;
            }}
        ",

        key,
        initializers,
        match_node(ir.clone(), &value)
    )
}

pub fn match_element_initializer(ir: Arc<AntlrIR>, element: &ElementIR, element_idx: usize) -> Option<String> {
    match element.suffix() {
        None | Some(EBNFSuffix::Optional) => Some(format!("{} = NULL;", element_decl(ir.clone(), element, element_idx)?)),
        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => Some(format!("Array* {} = (Array*) malloc(sizeof(Array));", element_name_indexed(ir, element, element_idx)?))
    }
}

pub fn match_element(ir: Arc<AntlrIR>, alt: HashArc<AltIR>, element: &ElementIR, element_idx: usize, next: &MatchNode) -> String {
    let element = match element {
        ElementIR::RuleAtom { id, suffix } => {
            let name = element_name(ir.clone(), element).unwrap();
            let name_indexed = element_name_indexed(ir.clone(), element, element_idx).unwrap();
            match suffix {
                None => 
                    format!("{} = parse_{}(parser);",
                        element_name_indexed(ir.clone(),
                        element,
                        element_idx).unwrap(),
                        name
                    ), //remember optional later

                Some(EBNFSuffix::Optional) => 
                    format!("{} = parse_{}(parser);",
                        element_name_indexed(ir.clone(),
                        element,
                        element_idx).unwrap(),
                        name
                    ), //remember optional later                
                
                Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => 
                    format!("{}* __item_{3} = NULL;
                        while ((__item_{3} = parse_{}(parser)) != NULL) {{ push({}, __item_{3}); }}", capitalize(element_name(ir.clone(), element).unwrap()),  name, name_indexed, element_idx)
            }
        },
        ElementIR::TokenAtom { id, suffix } => {
            match ir.symbols().get_token_name(*id) {
                Some(_) => {
                    let name_indexed = element_name_indexed(ir.clone(), element, element_idx).unwrap();
                    match suffix {
                        None => format!("{} = match_token(parser, {});", name_indexed, id),
                        Some(EBNFSuffix::Optional) => format!("{} = match_token(parser, {});", name_indexed, id), // Remember optional later
                        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!(
                            "Token* __item_{2} = NULL;
                        while ((__item_{2} = match_token(parser, {})) != NULL) {{ push({}, __item_{2}); }}", id, name_indexed, element_idx)
                    }
                    
                },
                None => {
                    match suffix {
                        None => format!("match_token(parser, {});", id),
                        Some(EBNFSuffix::Optional) => format!("match_token(parser, {});", id), // Remember optional later
                        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!("while (match_token(parser, {}) != NULL) {{}}", id)
                    }
                    
                }
            }
        },
        ElementIR::Set { set, suffix, inverted } => {
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
            "return new_{}_{}({});
            ", parent_rule.name().clone(), alt.label().cloned().unwrap_or(format!("alt{}", alt.index())), elements.join(",\n")
        )
    } else {
        let elements: Vec<_> = alt.elements().iter().enumerate().filter_map(
            |(e_idx, e)| element_name_indexed(ir.clone(), e, e_idx) ).collect();

        format!(
            "return new_{}({});
            ", parent_rule.name().clone(), elements.join(", ")
        )
    }
}