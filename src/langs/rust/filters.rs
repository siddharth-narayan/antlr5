use std::sync::Arc;

use minijinja::value::ViaDeserialize;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::MatchNode, intermediate::{AntlrIR, element::ElementIR}}};

pub fn element_prefix(e: ViaDeserialize<ElementIR>) -> String {
    match e.0 {
        ElementIR::RuleAtom { id, suffix } => {
            if let Some(suffix) = e.suffix() {
                match suffix {
                    EBNFSuffix::Optional => "Option<Box<".into(),
                    EBNFSuffix::Plus | EBNFSuffix::Star => "Vec<".into(),
                }
            } else {
                "Box<".into()
            }
        },

        ElementIR::TokenAtom { id, suffix } => {
            String::new()
        },

        ElementIR::Set { set, suffix } => {
            String::new()
        }
    }
}

pub fn element_suffix(e: ViaDeserialize<ElementIR>) -> String {
    match e.0 {
        ElementIR::RuleAtom { id, suffix } => {
            if let Some(suffix) = e.suffix() {
                match suffix {
                    EBNFSuffix::Optional => ">>".into(),
                    EBNFSuffix::Plus | EBNFSuffix::Star => ">".into(),
                }
            } else {
                ">".into()
            }
        },

        ElementIR::TokenAtom { id, suffix } => {
            String::new()
        },

        ElementIR::Set { set, suffix } => {
            String::new()
        }
    }
}

pub fn match_node_filter(ir: Arc<AntlrIR>) -> impl Fn(ViaDeserialize<MatchNode>) -> String {
    let ir = ir.clone();
    let filter = move |node: ViaDeserialize<MatchNode>| {
        let ir = ir.clone()  ;      
        match_node_internal(ir.clone(), &node)
    };

    filter
}

pub fn match_node_internal(ir: Arc<AntlrIR>, node: &MatchNode) -> String {
            match node {
                MatchNode::Peek { peek, fallback } => {
                    
                    let cases = peek.iter().map(|(key, value)| {
                        format!(
                            "Some({}) => {{
                                {}
                            }},
                            ",

                            key,
                            match_node_internal(ir.clone(), value)
                        )
                    }).collect::<Vec<_>>().join("\n");

                    format!(
                        "match self.peek().map(|t| t.token_type) {{
                            {}
                        }}
                        ",

                        cases
                    )
                },

                MatchNode::Element { alt, element_idx, element, next } => {
                    match element {
                        ElementIR::RuleAtom { id, suffix } => {
                            let n = ir.symbols().get_rule_name(*id).unwrap();

                            match suffix {
                                None => format!("let {} = self.{}();", n, n),
                                Some(EBNFSuffix::Optional) => format!("let {} = Some(self.{}());", n, n),
                                Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!("{}.push(self.{})", n, n)
                            }
                        },
                        ElementIR::TokenAtom { id, suffix } => {
                            match ir.symbols().get_token_name(*id) {
                                Some(n) => {
                                    match suffix {
                                        None => format!("let {} = self.{}();", n, n),
                                        Some(EBNFSuffix::Optional) => format!("let {} = Some(self.{}());", n, n),
                                        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!("{}.push(self.{})", n, n)
                                    }
                                    
                                },
                                None => String::new()
                            }
                        },
                        ElementIR::Set { set, suffix } => {
                            String::new()
                        }

                    }
                },

                MatchNode::Finish { alt } => {
                    let parent_rule = ir.get_rule(alt.parent()).unwrap();

                    if !parent_rule.alts().len() > 1 {
                        let elements: Vec<_> = alt.elements().iter().map(|e| {
                            match e {
                                ElementIR::RuleAtom { id, suffix } => {
                                    let name = ir.symbols().get_rule_name(*id).unwrap();

                                    match *suffix {
                                        None | Some(EBNFSuffix::Optional) => {
                                            format!("{}: {}.map(Box::new),", name, name)
                                        },

                                        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => {
                                            format!("{}: {},", name, name)
                                        },
                                    }
                                },
                                ElementIR::TokenAtom { id, suffix } => {
                                    match ir.symbols().get_token_name(*id) {
                                        Some(name) => format!("{},", name),
                                        None => String::new()
                                    }
                                },
                                ElementIR::Set { set, suffix } => String::new()
                            }
                        }).collect();

                        format!(
                            "{}::{} {{
                                {}
                            }}
                            ", parent_rule.name(), alt.label().cloned().unwrap_or(format!("Alt{}", alt.index())), elements.join("\n")
                        )
                    } else {
                        let elements: Vec<_> = alt.elements().iter().map(|e| {
                            match e {
                                ElementIR::RuleAtom { id, suffix } => {
                                    let name = ir.symbols().get_rule_name(*id).unwrap();

                                    match *suffix {
                                        None | Some(EBNFSuffix::Optional) => {
                                            format!("{}.map(Box::new),", name)
                                        },

                                        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => {
                                            format!("{},", name)
                                        },
                                    }
                                },
                                ElementIR::TokenAtom { id, suffix } => {
                                    match ir.symbols().get_token_name(*id) {
                                        Some(name) => format!("{},", name),
                                        None => String::new()
                                    }
                                },
                                ElementIR::Set { set, suffix } => String::new()
                            }
                        }).collect();

                        format!(
                            "{}::new (
                                {}
                            )
                            ", parent_rule.name(), elements.join("\n")
                        )
                    }            
                }
            }
        }
