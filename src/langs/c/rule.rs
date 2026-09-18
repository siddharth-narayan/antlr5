use std::{collections::HashMap, path::PathBuf, sync::Arc};

use rapidhash::fast::RandomState;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, langs::{OutputFile, c::{element::{element_decl, element_name_indexed}, r#match::{match_element_initializer, match_node}}}, util::{HashArc, capitalize}};


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

pub fn rule_decl(ir:Arc<AntlrIR>, rule_idx: usize) -> String {
    let parser_decl = rule_parser_decl(ir.clone(), rule_idx);
    let rule = ir.get_rule(rule_idx).unwrap();

    if rule.alts().len() == 1 {
        let mut ctor_decl = rule_struct_ctor_decl(ir.clone(), rule_idx);
        ctor_decl.push(';');

        let struct_decl = rule_struct_decl(ir, rule_idx);

        format!("{}{}{}", struct_decl, ctor_decl, parser_decl)
    } else {
        let enum_decl = rule_enum_decl(ir.clone(), rule_idx);

        let mut ctor_decls = (0..rule.alts().len()).into_iter().map(|alt_idx|
            rule_enum_ctor_decl(ir.clone(), rule_idx, alt_idx)
        ).collect::<Vec<_>>().join(";\n");
        ctor_decls.push(';');

        format!("{}{}{}", enum_decl, ctor_decls, parser_decl)
    }
}

pub fn rule_typedef(ir: Arc<AntlrIR>, rule_idx: usize) -> String {
    let rule = ir.get_rule(rule_idx).unwrap();
    let name = rule.name().clone();

    format!("typedef struct {0} {0};", capitalize(name.clone()))
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
    let alt = rule.alts().get(0).unwrap();
    let name = rule.name().clone();
    let elements_decls: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();
    
    format!(
        "{1}* new_{0}({2})", name, capitalize(name.clone()), elements_decls.join(", ")
    )
}

pub fn rule_enum_ctor_decl(ir: Arc<AntlrIR>, rule_idx: usize, alt_idx: usize) -> String {
    let rule = ir.get_rule(rule_idx).unwrap();
    let alt = rule.alts().get(alt_idx).unwrap();
    let name = rule.name().clone();
    let elements_decls: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();
    
    format!(
        "{1}* new_{0}_{3}({2})", name, capitalize(name.clone()), elements_decls.join(", "), alt.label().unwrap_or(&format!("alt{}", alt.index()))
    )
}

pub fn rule_ctor(ir: Arc<AntlrIR>, rule_idx: usize) -> String {
    let rule = ir.get_rule(rule_idx).unwrap();
    let name = rule.name().clone();
    let alt = rule.alts().get(0).unwrap();

    if rule.alts().len() > 1 {
        rule.alts().iter().map(|alt| {
            let label = alt.label().cloned().unwrap_or(format!("alt{}", alt.index()));

            let elements_assigns_vec: Vec<String> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| {
                Some(format!("__result->variants.{}.{} = {1};", label, element_name_indexed(ir.clone(), e, e_idx)?))
            }).collect();

            format!(
                "
                {0} {{
                    {1}* __result = ({1}*) malloc(sizeof({1}));
                    {2}

                    return __result;
                }}
                ", rule_enum_ctor_decl(ir.clone(), rule_idx, alt.index()), capitalize(name.clone()), elements_assigns_vec.join("\n")
            )
        }).collect()
    } else {
        let elements_assigns_vec: Vec<String> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| {
            Some(format!("__result->{} = {0};", element_name_indexed(ir.clone(), e, e_idx)?))
        }).collect();

        format!(
            "
            {0} {{
                {1}* __result = ({1}*) malloc(sizeof({1}));
                {2}
                
                return __result;
            }}
            ", rule_struct_ctor_decl(ir.clone(), rule_idx), capitalize(name.clone()), elements_assigns_vec.join("\n")
        )
    }

}

pub fn rule_enum_decl(ir: Arc<AntlrIR>, rule: usize) -> String { 
    let name = ir.get_rule(rule).unwrap().name().clone();

    let alts = ir.get_rule(rule).unwrap().alts().iter().map(|a| rule_enum_alt(ir.clone(), a.clone())).collect::<Vec<_>>().join(";");

    format!(
        "struct {} {{
            uint8_t tag;
            union {{
                {}
            }} variants;
        }};
        ", capitalize(name), alts
    )
}

pub fn rule_enum_alt(ir: Arc<AntlrIR>, alt: HashArc<AltIR>) -> String {
    let label = alt.label().cloned().unwrap_or(format!("alt{}", alt.index()));
    let elements = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect::<Vec<_>>().join(";\n");

    format!(
        "struct {{
            uint8_t;
            {}
        }} {};
        ", elements, label
    )
}