use serde::{Deserialize, Serialize};

use crate::{antlr::{ANTLRToken, ANTLRTokenType::{self, RuleID, TokenID}, Lexer, LexerErr::{self, EOF}}, ast::{ANTLRAst, alternative::{Alt, AltList}, ebnf::EBNFSuffix, rules::{Atom, Block, Element, Rule}}};



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

pub struct Parser {
    tokens: Vec<ANTLRToken>,
    head: usize
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Result<Parser, LexerErr> {
        let mut tokens = Vec::new();

        loop {
            let token = lexer.next_token()?;

            if token.token_type() == ANTLRTokenType::WS || token.token_type() == ANTLRTokenType::Comment || token.token_type() == ANTLRTokenType::CommentBlock{
                continue;
            }

            if token.token_type() == ANTLRTokenType::EOF {
                break;
            }
            
            println!("{:#?}", token);
            tokens.push(token);


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



impl Parser {
    pub fn grammar_spec(&mut self) -> Result<ANTLRAst, ParserErr> {
        let mut rules = Vec::new();

        loop {
            if let Some(t) = self.peek(1) {
                match t.token_type() {
                    ANTLRTokenType::TokenID => {
                        rules.push(self.token_rule_spec()?)
                    },

                    ANTLRTokenType::RuleID => {
                        rules.push(self.rule_spec()?)
                    },

                    ANTLRTokenType::EOF => {
                        break;
                    }

                    _ => {
                        return Err(ParserErr::NoTokenMatched { expected: vec![TokenID, RuleID, ANTLRTokenType::EOF], got: t.token_type() })
                    }
                }
            }
        }
        
        Ok(ANTLRAst::new(rules))
    }

    pub fn rule_spec(&mut self) -> Result<Rule, ParserErr> {
        let rule_name = self.match_token(ANTLRTokenType::RuleID)?.text().clone();
        
        self.match_token(ANTLRTokenType::Colon)?;

        let alts = self.alt_list()?;

        self.match_token(ANTLRTokenType::Semi)?;

        Ok(Rule::new(rule_name, alts))
    }

    pub fn token_rule_spec(&mut self) -> Result<TokenRule, ParserErr> {
        
    }

    pub fn block(&mut self) -> Result<Block, ParserErr> {
        self.match_token(ANTLRTokenType::LParen)?;
        let alts = self.alt_list()?;
        self.match_token(ANTLRTokenType::RParen)?;

        Ok(Block(alts))
    }

    pub fn element(&mut self) -> Result<Element, ParserErr> {
        // Elements are atom or block  and then a suffix?
        match self.peek_type(1).ok_or(ParserErr::UnexpectedEOF)? {
            ANTLRTokenType::StringLit => {
                let atom = self.atom()?;
                let suffix = self.ebnf_suffix().ok();
                Ok(Element::Atom { atom, suffix })
            },

            TokenID | RuleID => {
                let atom = self.atom()?;
                let suffix = self.ebnf_suffix().ok();
                Ok(Element::Atom { atom, suffix })
            },

            ANTLRTokenType::LParen => {
                let block = self.block()?;
                let suffix = self.ebnf_suffix().ok();

                Ok(Element::Block { block, suffix })
            }

            t => {
                Err(ParserErr::NoTokenMatched { expected: vec![ANTLRTokenType::StringLit, ANTLRTokenType::TokenID, ANTLRTokenType::RuleID, ANTLRTokenType::LParen], got: t })
            }
        }
    }

    // An Ok(None) result means an empty alt
    pub fn alt(&mut self) -> Result<Option<Alt>, ParserErr> {
        let mut elements = Vec::new();

        while let Ok(e) = self.element() {
            elements.push(e);
        };

        if elements.len() == 0 {
            return Ok(None);
        } else {
            let label = if self.match_token(ANTLRTokenType::Pound).is_ok() {
                Some(self.match_any_token(vec![ANTLRTokenType::RuleID, ANTLRTokenType::TokenID])?.text())
            } else {
                None
            };
            
            return Ok(Some(Alt::new(label, elements)))
        }
    }
       

    // Alts seperated by OR
    pub fn alt_list(&mut self) -> Result<AltList, ParserErr> {
        let mut optional = false;
        let mut alts = Vec::new();

        match self.alt()? {
            Some(a) => alts.push(a),
            None => optional = true,
        };

        while let Ok(_) = self.match_token(ANTLRTokenType::OR) {
            match self.alt()? {
                Some(a) => alts.push(a),
                None => optional = true,
            }
        };
        
        Ok(AltList::new(optional, alts))
    }

    pub fn atom(&mut self) -> Result<Atom, ParserErr> {
        let token = self.match_any_token(vec![ANTLRTokenType::StringLit, ANTLRTokenType::RuleID, ANTLRTokenType::TokenID])?;

        match token.token_type() {
            ANTLRTokenType::StringLit => Ok(Atom::StringLit(token.text().clone())),
            ANTLRTokenType::TokenID | ANTLRTokenType::RuleID => Ok(Atom::ID(token.text().clone())),
            _ => unreachable!()
        }
    }

    pub fn ebnf_suffix(&mut self) -> Result<EBNFSuffix, ParserErr> {
        let token =  self.match_any_token(vec![ANTLRTokenType::Question, ANTLRTokenType::Star, ANTLRTokenType::Plus])?;

        match token.token_type() {
            ANTLRTokenType::Question => Ok(EBNFSuffix::Optional),
            ANTLRTokenType::Star => {
                if self.match_token(ANTLRTokenType::Question).is_ok() {
                    Ok(EBNFSuffix::StarOptional)
                } else {
                    Ok(EBNFSuffix::Star)
                }
            },
            ANTLRTokenType::Plus => {
                if self.match_token(ANTLRTokenType::Question).is_ok() {
                    Ok(EBNFSuffix::PlusOptional)
                } else {
                    Ok(EBNFSuffix::Plus)
                }
            }
            _ => unreachable!()
        }
    }
}