use std::{collections::HashMap, sync::Arc};

use rapidhash::fast::RandomState;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, util::{HashArc, capitalize}};

pub fn render(ir: Arc<AntlrIR>) -> String {
    let parsers = (0..ir.rules().len()).map(|r| rule_parser(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    let types = (0..ir.rules().len()).map(|r| rule_type(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    
    format!(
        "
        {}
        impl Parser {{
            {}
        }}
        
        {}
        ", header(), parsers, types)
}

pub fn header() -> String {
    "
    use std::collections::VecDeque;
    
    pub enum ANTLRError {
        NoAlt,
        Mismatched,
        EOF
    }

    #[derive(Clone)]
    pub struct Token {
        token_type: usize,
        text: String
    }

    pub struct Parser {
        head: usize,
        tokens: Vec<Token>,
        rule_stack: VecDeque<usize>
    }

    impl Parser {
        pub fn peek(&self) -> Option<&Token> {
            self.tokens.get(self.head + 1)
        }

        pub fn match_token(&mut self, token: usize) -> Result<Token, ANTLRError> {
            match self.tokens.get(self.head) {
                Some(t) => {
                    if t.token_type == token {
                        self.head += 1;
                        Ok(t.clone())
                    } else {
                        Err(ANTLRError::Mismatched)
                    }
                },
                None => Err(ANTLRError::EOF)
            }
        }
    }
    ".into()
}

pub fn rule_parser(ir: Arc<AntlrIR>, rule: usize) -> String {
    let name = ir.get_rule(rule).unwrap().name().clone();
    format!(
        "pub fn {1}(&mut self) -> Result<{2}, ANTLRError> {{
            self.rule_stack.push_back({0});

            let __result = {{
                {3}
            }};

            self.rule_stack.pop_back();
            Ok(__result)
        }}",

        rule, name.clone(), capitalize(name), match_node(ir.clone(), &match_rule(ir, rule))
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

    let elements: Vec<_> = alt.elements().iter().map(|e| element_decl(ir.clone(), e)).collect();

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
        ", capitalize(name), elements.join("\n"), alt.elements().iter().map(|e| element_name(ir.clone(), e)).collect::<Vec<_>>().join("")
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
    let elements = alt.elements().iter().map(|e| element_decl(ir.clone(), e)).collect::<Vec<_>>().join("");

    format!(
        "{} {{
            {}
        }}
        ", label, elements
    )
}

pub fn element_name(ir: Arc<AntlrIR>, element: &ElementIR) -> String {
    match element {
        ElementIR::RuleAtom { id, suffix } => {
            format!("{},", ir.rules().get(*id).unwrap().name().clone())
        },

        ElementIR::TokenAtom { id, suffix } => {
            if let Some(name) = ir.symbols().get_token_name(*id) {
                format!("{},", name.clone())
            } else {
                String::new()
            }
        },

        ElementIR::Set { set, suffix } => {
            String::new()
        }
    }
}

pub fn element_decl(ir: Arc<AntlrIR>, element: &ElementIR) -> String {
    match element {
        ElementIR::RuleAtom { id, suffix } => {
            let name = ir.rules().get(*id).unwrap().name();

            let prefix: String = match element.suffix() {
                None => "Box<".into(),
                Some(EBNFSuffix::Optional) => "Option<Box<".into(),
                Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => "Vec<".into()
            };

            let suffix: String = match element.suffix() {
                Some(EBNFSuffix::Optional) => ">>".into(),
                None | Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => ">".into()
            };

            format!(
                "{}: {}{}{},", name, prefix, capitalize(name.clone()), suffix
            )
        },

        ElementIR::TokenAtom { id, suffix } => {
            if let Some(name) = ir.symbols().get_token_name(*id) {
                let prefix: String = match element.suffix() {
                    None => "".into(),
                    Some(EBNFSuffix::Optional) => "Option<".into(),
                    Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => "Vec<".into()
                };

                let suffix: String = match element.suffix() {
                    None => "".into(),
                    _  => ">".into(),
                };

                format!(
                    "{}: {}Token{},", name, prefix , suffix
                )
            } else {
                String::new()
            }
        },

        ElementIR::Set { set, suffix } => {
            String::new()
        }
    }
}

pub fn match_peek(ir: Arc<AntlrIR>, peek: &HashMap<usize, MatchNode, RandomState>, fallback: Option<&Box<MatchNode>>) -> String {
    let cases = peek.iter().map(|(key, value)| {
        match_case(ir.clone(), *key, value)
    }).collect::<Vec<_>>().join("\n");

    format!(
        "match self.peek().map(|t| t.token_type) {{
            _ => panic!(),
            {}
        }}
        ",

        cases
    )
}

pub fn match_case(ir: Arc<AntlrIR>, key: usize, value: &MatchNode) -> String {
    format!(
        "
            Some({}) => {{
                {}
            }},
        ",

        key,
        match_node(ir.clone(), &value)
    )
}

pub fn match_node(ir: Arc<AntlrIR>, node: &MatchNode) -> String {
    match node {
        MatchNode::Peek { peek, fallback } => match_peek(ir.clone(), peek, fallback.as_ref()),
        MatchNode::Element { alt, element_idx, element, next } => match_element(ir, alt.clone(), element, *element_idx, next),
        MatchNode::Finish { alt } => match_finish(ir, alt.clone()),
    }
}

pub fn match_element_initializer(ir: Arc<AntlrIR>, element: &ElementIR) -> String {
    match element {
        ElementIR::RuleAtom { id, suffix } => {
            let name = ir.get_rule(*id).unwrap().name().clone();

            match suffix {
                None => String::new(),
                Some(EBNFSuffix::Optional) => format!("let {} = None;", name),
                Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!("let mut {} = Vec::new();", name)
            }
        },

        ElementIR::TokenAtom { id, suffix } => {
            match ir.symbols().get_token_name(*id) {
                Some(name) => {
                    match suffix {
                        None => String::new(),
                        Some(EBNFSuffix::Optional) => format!("let {} = None;", name),
                        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!("let mut {} = Vec::new();", name)
                    } 
                },
                None => String::new()
            }
        },

        ElementIR::Set { set, suffix } => String::new()
    }
}

pub fn match_element(ir: Arc<AntlrIR>, alt: HashArc<AltIR>, element: &ElementIR, element_idx: usize, next: &MatchNode) -> String {
    let initializers = if element_idx == 0 {
        alt.elements().iter().map(|e| match_element_initializer(ir.clone(), e)).collect::<Vec<_>>().join("\n")
    } else {
        String::new()
    };

    let element = match element {
        ElementIR::RuleAtom { id, suffix } => {
            let n = ir.get_rule(*id).unwrap().name().clone();

            match suffix {
                None => format!("let {} = Box::new(self.{}()?);", n, n),
                Some(EBNFSuffix::Optional) => format!("let {} = self.{}().ok().map(Box::new);", n, n),
                Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!("while let Ok(__item) = self.{}() {{ {}.push(__item) }}", n, n)
            }
        },
        ElementIR::TokenAtom { id, suffix } => {
            match ir.symbols().get_token_name(*id) {
                Some(n) => {
                    match suffix {
                        None => format!("let {} = self.match_token({})?.clone();", n, id),
                        Some(EBNFSuffix::Optional) => format!("let {} = self.match_token({}).ok().map(Box::new());", n, n),
                        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => format!("while let Ok(x) = self.match_token({}) {{ {}.push(x) }}", id, n)
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

    format!("{}\n{}\n{}", initializers, element, match_node(ir, &next))
}

pub fn match_finish(ir: Arc<AntlrIR>, alt: HashArc<AltIR>) -> String {
  let parent_rule = ir.get_rule(alt.parent()).unwrap();

    if parent_rule.alts().len() > 1 {
        let elements: Vec<_> = alt.elements().iter().map(|e| {
            match e {
                ElementIR::RuleAtom { id, suffix } => {
                    let name = ir.get_rule(*id).unwrap().name().clone();

                    match *suffix {
                        None | Some(EBNFSuffix::Optional) => {
                            format!("{},", name)
                        },

                        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => {
                            format!("{},", name)
                        },
                    }
                },
                ElementIR::TokenAtom { id, suffix } => {
                    match ir.symbols().get_token_name(*id) {
                        Some(name) => format!("{},", name),
                        None => "// strlit (probably)".into()
                    }
                },
                ElementIR::Set { set, suffix } => "// Set".into()
            }
        }).collect();

        format!(
            "{}::{} {{
                {}
            }}
            ", capitalize(parent_rule.name().clone()), alt.label().cloned().unwrap_or(format!("Alt{}", alt.index())), elements.join("\n")
        )
    } else {
        let elements: Vec<_> = alt.elements().iter().map(|e| {
            match e {
                ElementIR::RuleAtom { id, suffix } => {
                    let name = ir.get_rule(*id).unwrap().name().clone();

                    match *suffix {
                        None | Some(EBNFSuffix::Optional) => {
                            format!("{},", name)
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
            ", capitalize(parent_rule.name().clone()), elements.join("\n")
        )
    }
}