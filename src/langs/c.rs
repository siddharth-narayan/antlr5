use std::{collections::HashMap, path::PathBuf, sync::Arc};

use rapidhash::fast::RandomState;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, langs::OutputFile, util::{HashArc, capitalize}};

pub fn render(ir: Arc<AntlrIR>, mut dir: PathBuf) -> Vec<OutputFile> {
    dir.absolute().unwrap();
    if dir.is_file() {
        dir = dir.parent().unwrap().to_path_buf()
    }

    let source_path = dir.join("out.c");
    let header_path = dir.join("out.h");

    let parsers = (0..ir.rules().len()).map(|r| rule_parser(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    let types = (0..ir.rules().len()).map(|r| rule_type(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    
    let header = format!(
        "{}
        ", types
    );

    let source = 
        format!(
            "
            {}
            {}
            {}
            ", source_file_header(&header_path), parsers, types
        );

    vec![
        OutputFile {
            path: source_path,
            content: source,
        },

        OutputFile {
            path: header_path,
            content: header,
        }
    ]
}

pub fn source_file_header(header_path: &PathBuf) -> String {
    format!(
        "#include <stdint.h>
        #include <errno.h>

        #include \"{}\"

        typedef struct {{
            uint32_t token_type;

            char* text;
            uint32_t text_len;
        }} Token;

        typedef struct Parser {{
            uint32_t head;
            uint32_t token_count;
            Token*   tokens;
        }} Parser;

        Token* get_token(Parser* parser, uint32_t index) {{
            if (parser->token_count <= index) {{
                errno = ERANGE;
                return NULL;
            }}

            return parser->tokens + index;
        }}

        Token* peek(Parser* parser) {{
            return get_token(parser, parser->head + 1);
            }}

        Token* match_token(Parser* parser, uint32_t token_type) {{
            Token* token = get_token(parser, parser->head);
            if (token == NULL) {{
                return NULL;
            }}

            if (token->token_type == token_type) {{
                parser->head += 1;
                return token;
            }}

            return NULL;
        }}
        ", header_path.to_str().unwrap()
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
        "{1}* {0}(Parser* parser) {{
            {1}* __result = ({1} *) malloc(sizeof({1});
            {2}
        }}",

        name.clone(), capitalize(name), match_node(ir.clone(), &rule_match)
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
        "typedef struct {{
            {2}
        }} {1};

         {1}* new_{0}({2}) {{
                {0} {{
                    {2}
                }}
            }}
        }}
        ", name, capitalize(name.clone()), elements.join(",\n")
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
            return NULL
        }}
        
        switch (next_token->token_type) {{
            {}
            _ => panic!(),
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
            case {}:
                {}
                break;
        ",

        key,
        match_node(ir.clone(), &value)
    )
}

pub fn match_element_initializer(ir: Arc<AntlrIR>, element: &ElementIR, element_idx: usize) -> Option<String> {
    match element.suffix() {
        None => None,
        Some(EBNFSuffix::Optional) => Some(format!("{} = NULL;", element_decl(ir.clone(), element, element_idx)?)),
        Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => Some(format!("{} = ({}*) malloc(sizeof(Array);", element_decl(ir.clone(), element, element_idx)?, element_name(ir, element)?))
    }
}

pub fn match_element(ir: Arc<AntlrIR>, alt: HashArc<AltIR>, element: &ElementIR, element_idx: usize, next: &MatchNode) -> String {
    let name = element_name(ir.clone(), element).unwrap();
    let name_indexed = element_name_indexed(ir.clone(), element, element_idx).unwrap();

    let element = match element {
        ElementIR::RuleAtom { id, suffix } => {
            match suffix {
                None => format!("{} = {}(parser);", name_indexed, name),
                Some(EBNFSuffix::Optional) => format!("{} = {}(parser);", name_indexed, name),
                Some(EBNFSuffix::Plus) | Some(EBNFSuffix::Star) => 
                format!("{}* item = NULL;

                while ({}* __item = {}(parser) != NULL) {{ push({}, __item) }}", capitalize(name.clone()), capitalize(name.clone()), name, name)
            }
        },
        ElementIR::TokenAtom { id, suffix } => {
            match ir.symbols().get_token_name(*id) {
                Some(_) => {
                    match suffix {
                        None => format!("let {} = match_token(parser, {})?.clone();", name_indexed.unwrap(), id),
                        Some(EBNFSuffix::Optional) => format!("let {} = self.match_token({}).ok().map(Box::new());", name_indexed.unwrap(), id),
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