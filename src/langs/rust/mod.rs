use std::{collections::HashMap, sync::Arc};

use rapidhash::fast::RandomState;

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, langs::{OutputFile, rust::{lexer::lexer_match, rule::{rule_parser, rule_type}}}, util::{HashArc, capitalize}};

mod element;
mod r#match;
mod rule;

mod lexer;

pub fn render(ir: Arc<AntlrIR>) -> Vec<OutputFile> {
    let parsers = (0..ir.rules().len()).map(|r| rule_parser(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    let types = (0..ir.rules().len()).map(|r| rule_type(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    
    let lexer_match = lexer_match(ir.clone());
    vec![
        OutputFile {
            path: "parser.rs".into(),
            content: format!(
                "
                {}
                impl Parser {{
                    {}
                }}
                
                {}
                ", parse_header(), parsers, types
            )
        },

        OutputFile {
            path: "lexer.rs".into(),
            content: format!(
                "
                {}

                impl Lexer {{
                    pub fn next() -> Token {{
                        {}
                    }}
                }}
                ", lexer_header(), lexer_match
            )
        }
    ]

}

pub fn parse_header() -> String {
    "#![allow(unused, nonstandard_style)]
    use std::collections::VecDeque;
    
    #[derive(Clone, Debug)]
    pub enum ANTLRError {
        NoViableAlt,
        Mismatched,
        EOF
    }

    #[derive(Clone, Debug)]
    pub struct Token {
        token_type: usize,
        text: String
    }

    #[derive(Clone, Debug)]
    pub struct Parser {
        head: usize,
        tokens: Vec<Token>,
        rule_stack: VecDeque<usize>
    }

    impl Parser {
        pub fn new(tokens: Vec<Token>) -> Parser {
            Parser {
                head: 0,
                tokens,
                rule_stack: VecDeque::new()
            }
        }

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

pub fn lexer_header() -> String {
    "#![allow(unused, nonstandard_style)]
    use std::collections::VecDeque;

    #[derive(Clone, Debug)]
    pub struct Token {
        token_type: usize,
        text: String
    }

    #[derive(Clone, Debug)]
    pub struct Lexer {
        head: usize,
        text: Vec<char>,
        rule_stack: VecDeque<usize>
    }

    impl Lexer {
        pub fn new(string: String) -> Lexer {
            let text = string.chars().collect();

            Lexer { head: 0, text }
        }
    }
    ".into()
}