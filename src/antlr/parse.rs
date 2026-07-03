use crate::antlr::{ANTLRToken, ANTLRTokenType::{self, ID}, Lexer, LexerErr::{self, EOF}, parse::rules::Rule};

mod rules;
mod ebnf;
mod r#impl;
mod alternative;

#[derive(Debug)]
pub enum ParserErr {
    UnexpectedToken { 
        expected: ANTLRTokenType,
        got: ANTLRTokenType
    },
    NoTokenMatched {
        expected: Vec<ANTLRTokenType>,
        got: ANTLRTokenType
    },
    SyntaxErr {
        reason: String
    },
    UnexpectedEOF
}

#[derive(Debug)]
pub struct ANTLRAst {
    rules: Vec<Rule>
}

impl ANTLRAst {
    pub fn new(rules: Vec<Rule>) -> ANTLRAst {
        ANTLRAst { rules }
    }
}

pub struct Parser {
    tokens: Vec<ANTLRToken>,
    head: usize
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Parser, LexerErr> {
        let mut tokens = Vec::new();

        loop {
            match lexer.next_token() {
                Ok(t) => {
                    if t.token_type() != ANTLRTokenType::WS {
                        tokens.push(t)
                    }
                },
                Err(EOF) => break,
                Err(e) => return Err(e)
            }
        };

        Ok(Parser {
            tokens,
            head: 0
        })
    }

    pub fn next(&mut self) -> Option<ANTLRToken> {
        let res = self.tokens.get(self.head);
        self.head += 1;

        res.cloned()
    }

    pub fn consume(&mut self, amount: usize) {
        self.head += amount
    }

    pub fn peek(&self, by: usize) -> Option<ANTLRToken> {
        self.tokens.get(self.head + by - 1).cloned()
    }

    pub fn peek_type(&self, by: usize) -> Option<ANTLRTokenType> {
        self.tokens.get(self.head + by - 1).map(|t| t.token_type())
    }

    pub fn peek_n(&self, by: usize, count: usize) -> Option<&[ANTLRToken]> {
        self.tokens
            .get((self.head + by - 1)..(self.head + by + count - 1))
    }

    pub fn match_token(&mut self, token_type: ANTLRTokenType) -> Result<ANTLRToken, ParserErr> {
        let peek = self.peek(1).ok_or(ParserErr::UnexpectedEOF)?;
        if peek.token_type() != token_type {
            return Err(ParserErr::UnexpectedToken {
                expected: token_type,
                got: peek.token_type()
            })
        };

        self.consume(1);

        Ok(peek)
    }

    pub fn match_any_token(&mut self, token_types: Vec<ANTLRTokenType>) -> Result<ANTLRToken, ParserErr> {
        let peek = self.peek(1).ok_or(ParserErr::UnexpectedEOF)?;

        for token_type in &token_types {
            if &peek.token_type() == token_type {
                self.consume(1);
                return Ok(peek)
            };
        }

        return Err(ParserErr::NoTokenMatched {
            expected: token_types,
            got: peek.token_type()
        })
        
    }
}

