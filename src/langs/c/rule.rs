use std::{collections::HashMap, path::PathBuf, sync::Arc};

use rapidhash::fast::RandomState;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, langs::{OutputFile, c::{element::element_decl, r#match::{match_element_initializer, match_node}}}, util::{HashArc, capitalize}};


pub fn rule_parser_decl(ir: Arc<AntlrIR> , rule: usize) -> String {
    let name = ir.get_rule(rule).unwrap().name().clone();

    format!(
        "{1}* parse_{0}(Parser* parser);",
        name.clone(), capitalize(name)
    )
}

pub fn rule_parser(ir: Arc<AntlrIR>, rule: usize) -> String {
    let name = ir.get_rule(rule).unwrap().name().clone();
    let rule_match = match_rule(ir.clone(), rule);

    let initializers = if let MatchNode::Element { alt, .. } = &rule_match {
        format!("{}", alt.elements().iter().enumerate().filter_map(|(e_idx, e)| match_element_initializer(ir.clone(), e, e_idx)).collect::<Vec<_>>().join("\n"))       
    } else {
        String::new()
    };

    format!(
        "{1}* parse_{0}(Parser* parser) {{
            {3}
            {2}
        }}",

        name.clone(), capitalize(name), match_node(ir.clone(), &rule_match), initializers
    )
}

pub fn rule_type(ir:Arc<AntlrIR>, rule: usize) -> String {
    if ir.get_rule(rule).unwrap().alts().len() == 1 {
        rule_struct_decl(ir, rule)
    } else {
        rule_enum(ir, rule)
    }
}

pub fn rule_typedef(ir: Arc<AntlrIR>, rule_idx: usize) -> String {
    let rule = ir.get_rule(rule_idx).unwrap();
    let name = rule.name().clone();

    format!("typedef struct {0} {0};", capitalize(name.clone())
    )
}

pub fn rule_struct_decl(ir: Arc<AntlrIR>, rule_idx: usize) -> String {
    let rule = ir.get_rule(rule_idx).unwrap();
    let name = rule.name().clone();
    let alt = rule.alts().get(0).unwrap();

    let elements_vec: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();
    let mut elements= elements_vec.join(";\n");
    if elements_vec.len() > 0 {
        elements.push(';');
    }

    // Empty uint8_t for padding empty structs
    format!(
        "struct {} {{
            uint8_t;
            {}
        }};
        ", capitalize(name.clone()), elements
    )
}

pub fn rule_struct_ctor_decl(ir: Arc<AntlrIR>, rule_idx: usize) -> String {
    let rule = ir.get_rule(rule_idx).unwrap();
    let name = rule.name().clone();


    if rule.alts().len() > 1 {
        rule.alts().iter().map(|alt| {
            let elements_decls: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();
            format!(
                "{1}* new_{0}_alt{3}({2})", name, capitalize(name.clone()), elements_decls.join(", "), alt.index()
            )
        }).collect()
    } else {
        let alt = rule.alts().get(0).unwrap();

        let elements_decls: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();
        format!(
            "{1}* new_{0}({2})", name, capitalize(name.clone()), elements_decls.join(", ")
        )
    }

}

pub fn rule_struct_ctor(ir: Arc<AntlrIR>, rule_idx: usize) -> String {
    let rule = ir.get_rule(rule_idx).unwrap();
    let name = rule.name().clone();
    let alt = rule.alts().get(0).unwrap();

    if rule.alts().len() > 1 {
        rule.alts().iter().map(|alt| {
            let elements_decls_vec: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();
            let mut element_decls = elements_decls_vec.join(";\n");
            if elements_decls_vec.len() > 0 {
                element_decls.push(';');
            }

            format!(
                "
                {} {{
                    return {} {{
                        {}
                    }};
                }}
                ", rule_struct_ctor_decl(ir.clone(), rule_idx), capitalize(name.clone()), element_decls
            )
        }).collect()
    } else {
        let elements_decls_vec: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();
        let mut element_decls = elements_decls_vec.join(";\n");
        if elements_decls_vec.len() > 0 {
            element_decls.push(';');
        }

        format!(
            "
            {} {{
                {} {{
                    {}
                }}
            }}
            ",  rule_struct_ctor_decl(ir.clone(), rule_idx), capitalize(name.clone()), element_decls
        )
    }

}

pub fn rule_enum(ir: Arc<AntlrIR>, rule: usize) -> String { 
    let name = ir.get_rule(rule).unwrap().name().clone();

    let alts = ir.get_rule(rule).unwrap().alts().iter().map(|a| rule_enum_alt(ir.clone(), a.clone())).collect::<Vec<_>>().join(",");
    format!(
        "pub enum {} {{
            {}
        }}
        ", capitalize(name), alts
    )
}

pub fn rule_enum_alt(ir: Arc<AntlrIR>, alt: HashArc<AltIR>) -> String {
    let label = alt.label().cloned().unwrap_or(format!("Alt{}", alt.index()));
    let elements = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect::<Vec<_>>().join(",");

    format!(
        "{} {{
            {}
        }}
        ", label, elements
    )
}