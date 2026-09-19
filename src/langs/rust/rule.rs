use std::sync::Arc;

use crate::{codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR}}, langs::rust::{element::{element_decl, element_name_indexed}, r#match::{match_element_initializer, match_node}}, util::{HashArc, capitalize}};

pub fn rule_parser(ir: Arc<AntlrIR>, rule: usize) -> String {
    let name = ir.get_rule(rule).unwrap().name().clone();
    let rule_match = match_rule(ir.clone(), rule);

    let initializers = if let MatchNode::Element { alt, .. } = &rule_match {
        format!("{}", alt.elements().iter().enumerate().filter_map(|(e_idx, e)| match_element_initializer(ir.clone(), e, e_idx)).collect::<Vec<_>>().join("\n"))       
    } else {
        String::new()
    };

    format!(
        "pub fn {1}(&mut self) -> Result<{2}, ANTLRError> {{
            self.rule_stack.push_back({0});

            let __result = {{
                {3}
                {4}
            }};

            self.rule_stack.pop_back();
            Ok(__result)
        }}",

        rule, name.clone(), capitalize(name), initializers, match_node(ir.clone(), &rule_match)
    )
}

pub fn rule_type(ir:Arc<AntlrIR>, rule: usize) -> String {
    if ir.get_rule(rule).unwrap().alts().len() == 1 {
        rule_struct(ir, rule)
    } else {
        rule_enum(ir, rule)
    }
}

pub fn rule_struct(ir: Arc<AntlrIR>, rule_idx: usize) -> String {
    let rule = ir.get_rule(rule_idx).unwrap();
    let name = rule.name().clone();
    let alt = rule.alts().get(0).unwrap();

    let elements: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();

    format!(
        "pub struct {0} {{
            {1}
        }}

        impl {0} {{
            pub fn new({1}) -> {0} {{
                {0} {{
                    {2}
                }}
            }}
        }}
        ", capitalize(name), elements.join(",\n"), alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_name_indexed(ir.clone(), e, e_idx)).collect::<Vec<_>>().join(",")
    )
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