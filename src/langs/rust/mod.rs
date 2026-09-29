use std::{collections::HashMap, sync::Arc};

use rapidhash::fast::RandomState;
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::{MatchNode, match_rule}, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, langs::{OutputFile, rust::{lexer::lexer_tokens, rule::{rule_parser, rule_type}}}, util::{HashArc, capitalize}};

mod element;
mod r#match;
mod rule;

mod lexer;

pub fn render(ir: Arc<AntlrIR>) -> Vec<OutputFile> {
    let parsers = (0..ir.rules().len()).into_par_iter().map(|r| rule_parser(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    let types = (0..ir.rules().len()).into_par_iter().map(|r| rule_type(ir.clone(), r)).collect::<Vec<_>>().join("\n");
    
    let lexer_tokens = lexer_tokens(ir.clone());
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

                {}
                
                impl Lexer {{
                    pub fn next() -> Token {{
                    }}
                }}
                ", lexer_header(), lexer_tokens
            )
        }
    ]

}

pub fn parse_header() -> String {
    "#![allow(unused, nonstandard_style)]
    use std::collections::VecDeque;
    
    mod lexer;
    use lexer::Token;

    #[derive(Clone, Debug)]
    pub enum ANTLRErrorType {
        EOF,
        NoViableAlt,
        Mismatched
    }

    #[derive(Clone, Debug)]
    pub struct ANTLRError {
        stack: std::collections::VecDeque<usize>,

        position: usize,
        error_type: ANTLRErrorType,
    }

    #[derive(Clone, Debug)]
    pub struct Parser {
        head: usize,
        tokens: std::vec::Vec<Token>,
        rule_stack: std::collections::VecDeque<usize>,
    }

    impl Parser {
        pub fn new(tokens: Vec<Token>) -> Parser {
            Parser {
                head: 0,
                tokens,
                rule_stack: std::collections::VecDeque::new(),
            }
        }

        pub fn err_eof(&self) -> ANTLRError {
            ANTLRError {
                stack: self.rule_stack.clone(),

                position: usize::MAX,
                error_type: ANTLRErrorType::EOF
            }
        }

        pub fn err_noviablealt(&self) -> ANTLRError {
            ANTLRError {
                stack: self.rule_stack.clone(),

                position: self.peek().map(|t| t.position()).unwrap_or_default(),
                error_type: ANTLRErrorType::NoViableAlt
            }
        }

        pub fn err_mismatched(&self) -> ANTLRError {
            ANTLRError {
                stack: self.rule_stack.clone(),

                position: self.peek().map(|t| t.position()).unwrap_or_default(),
                error_type: ANTLRErrorType::Mismatched
            }
        }

        pub fn peek(&self) -> std::option::Option<&Token> {
            self.tokens.get(self.head + 1)
        }

        pub fn match_token(&mut self, token: usize) -> Result<Token, ANTLRError> {
            match self.tokens.get(self.head) {
                Some(t) => {
                    if t.token_type() as usize == token {
                        self.head += 1;
                        Ok(t.clone())
                    } else {
                        Err(self.err_mismatched())
                    }
                }
                None => Err(self.err_eof()),
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
        token_type: TokenType,
        text: String,

        pos: usize,
        line_number: usize,
        col_number: usize,
    }
    
    impl Token {
        pub fn token_type(&self) -> TokenType {
          self.token_type
        }

        pub fn position(&self) -> usize {
          self.pos
        }
    }

    #[derive(Clone, Debug)]
    pub struct Lexer {
        head: usize,
        select_head: usize,
        
        text: Vec<char>,
        rule_stack: std::collections::VecDeque<usize>
    }

    impl Lexer {
        pub fn new(string: String) -> Lexer {
            let text = string.chars().collect();

            Lexer { head: 0, select_head: 0, text, rule_stack: std::collections::VecDeque::new() }
        }
    }
    ".into()
}