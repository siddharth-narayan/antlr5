use std::{collections::HashMap, path::PathBuf, sync::Arc};

use rapidhash::fast::RandomState;
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, langs::{OutputFile, c::rule::{rule_ctor, rule_decl, rule_parser, rule_parser_decl, rule_struct_decl, rule_typedef}}, util::{HashArc, capitalize}};

mod element;
mod r#match;
mod rule;

pub fn render(ir: Arc<AntlrIR>, mut dir: PathBuf) -> Vec<OutputFile> {
    dir.absolute().unwrap();
    
    if dir.extension().is_some() {
        dir = dir.parent().unwrap().to_path_buf()
    }

    let source_path = dir.join("out.c");
    let header_path = dir.join("out.h");



    // Header file
    let rule_typedefs = (0..ir.rules().len()).into_par_iter().map(|r| rule_typedef(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    let header_rule_decls = (0..ir.rules().len()).into_par_iter().map(|r| rule_decl(ir.clone(), r))
        .collect::<Vec<_>>().join("\n");

    let header = format!(
        "{}
        {}
        {}
        ", header_file_header(), rule_typedefs, header_rule_decls,
    );

    // Source File
    let parsers = (0..ir.rules().len()).into_par_iter().map(|r| rule_parser(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    let rule_ctors = (0..ir.rules().len()).into_par_iter().map(|r| rule_ctor(ir.clone(), r)).collect::<Vec<_>>().join("\n");


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
        "#include <stdlib.h>
        #include <stddef.h>
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

        void push(Array* array, void* element) {{
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

