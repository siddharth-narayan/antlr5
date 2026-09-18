use std::{collections::HashMap, path::PathBuf, sync::Arc};

use rapidhash::fast::RandomState;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, langs::OutputFile, util::{HashArc, capitalize}};

pub fn render(ir: Arc<AntlrIR>, mut dir: PathBuf) -> Vec<OutputFile> {
    dir.absolute().unwrap();
    
    if dir.extension().is_some() {
        dir = dir.parent().unwrap().to_path_buf()
    }

    let source_path = dir.join("out.c");
    let header_path = dir.join("out.h");

    let parsers = (0..ir.rules().len()).map(|r| rule_parser(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    let rule_ctors = (0..ir.rules().len()).map(|r| rule_struct_ctor(ir.clone(), r)).collect::<Vec<_>>().join("\n");



    let rule_typedefs = (0..ir.rules().len()).map(|r| rule_typedef(ir.clone(), r)).collect::<Vec<_>>().join("\n");

    let header_rule_decls = (0..ir.rules().len()).map(
        |r| 
        
        format!(
            "{}
            {}
            {}",
            rule_struct_decl(ir.clone(), r),
            rule_parser_decl(ir.clone(), r),
            rule_struct_ctor_decl(ir.clone(), r)
        )
    )
        .collect::<Vec<_>>().join("\n");

    let header = format!(
        "{}
        {}
        {}
        ", header_file_header(), rule_typedefs, header_rule_decls,
    );

    let source = 
        format!(
            "
            {}
            {}
            {}
            ", source_file_header(&header_path), parsers, rule_ctors
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
        "#include <stddef.h>
        #include <stdint.h>
        #include <errno.h>

        #include \"{}\"

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

pub fn header_file_header() -> String {
    "typedef struct Token Token;
    typedef struct Parser Parser;
    typedef struct Array Array;

    struct Token {
    uint32_t token_type;

    char *text;
    uint32_t text_len;
    };

    struct Parser {
    uint32_t head;
    uint32_t token_count;
    Token *tokens;
    };
    
    struct Array {
        void** head;
    };
    ".into()
}

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

    let elements: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();
    let mut elements= elements.join(";\n");
    elements.push(';');

    format!(
        "struct {} {{
            {}
        }};
        ", capitalize(name.clone()), elements
    )
}

pub fn rule_struct_ctor_decl(ir: Arc<AntlrIR>, rule_idx: usize) -> String {
    let rule = ir.get_rule(rule_idx).unwrap();
    let name = rule.name().clone();
    let alt = rule.alts().get(0).unwrap();

    let elements_decls: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();

    format!(
        "{1}* new_{0}({2});", name, capitalize(name.clone()), elements_decls.join(", ")
    )
}

pub fn rule_struct_ctor(ir: Arc<AntlrIR>, rule_idx: usize) -> String {
    let rule = ir.get_rule(rule_idx).unwrap();
    let name = rule.name().clone();
    let alt = rule.alts().get(0).unwrap();

    let elements_decls: Vec<_> = alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_decl(ir.clone(), e, e_idx)).collect();
    let mut element_decls = elements_decls.join(";\n");
    element_decls.push(';');

    format!(
        "
        {1}* new_{0}({2}) {{
            {1} {{
                {3}
            }}
        }}
        ", name, capitalize(name.clone()), element_decls, alt.elements().iter().enumerate().filter_map(|(e_idx, e)| element_name_indexed(ir.clone(), e, e_idx)).collect::<Vec<_>>().join(",")
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
    
    Some(format!("{} {}", element_type(ir.clone(), element), name))
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
            case {}:
                {}
                {}
                break;
        ",

        key,
        initializers,
        match_node(ir.clone(), &value)
    )
}

pub fn match_element_initializer(ir: Arc<AntlrIR>, element: &ElementIR, element_idx: usize) -> Option<String> {
    match element.suffix() {
        None | Some(EBNFSuffix::Optional) => Some(format!("{}_{} = NULL;", element_decl(ir.clone(), element, element_idx)?, element_idx)),
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
                    format!("{}* __item = NULL;
                        while ((__item = parse_{}(parser)) != NULL) {{ push({}, __item); }}", capitalize(element_name(ir.clone(), element).unwrap()),  name_indexed, name_indexed)
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
                            "Token* __item = NULL;
                        while (__item = match_token(parser, {}) != NULL) {{ push({}, __item); }}", id, name_indexed)
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
            "return new_{}alt_{}({});
            ", parent_rule.name().clone(), alt.label().cloned().unwrap_or(format!("Alt{}", alt.index())), elements.join(",\n")
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