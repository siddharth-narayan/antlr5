use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{antlr::lex::{ANTLRToken, ANTLRTokenType::{self, *}, Lexer, LexerErr}, ast::{ANTLRAst, alternative::{Alt, AltList}, ebnf::EBNFSuffix, rules::{Atom, Block, Element, Rule, TokenRule}}};




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

            if token.token_type() == WS || token.token_type() == Comment || token.token_type() == CommentBlock{
                continue;
            }

            if token.token_type() == EOF {
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

    pub fn next(&mut self) -> ANTLRToken {
        let res = self.tokens.get(self.head).cloned().unwrap_or(ANTLRToken::new(EOF, String::new()));
        self.head += 1;

        res
    }

    pub fn consume(&mut self, amount: usize) {
        self.head += amount
    }

    pub fn peek(&self, by: usize) -> ANTLRToken {
        self.tokens.get(self.head + by - 1).cloned().unwrap_or(ANTLRToken::new(EOF, String::new()))
    }

    pub fn peek_type(&self, by: usize) -> ANTLRTokenType {
        self.tokens.get(self.head + by - 1).map(|t| t.token_type()).unwrap_or(EOF)
    }

    pub fn peek_n(&self, by: usize, count: usize) -> Option<&[ANTLRToken]> {
        self.tokens
            .get((self.head + by - 1)..(self.head + by + count - 1))
    }

    pub fn match_token(&mut self, token_type: ANTLRTokenType) -> Result<ANTLRToken, ParserErr> {
        let peek = self.peek(1);
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
        let peek = self.peek(1);

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
        let mut token_rules = Vec::new();
        loop {
            match self.peek(1).token_type() {
                TokenID |  Fragment => {
                    match self.token_rule_spec() {
                        Ok(r) => token_rules.push(r),
                        Err(e) => {
                            println!("{:#?}", self.peek_n(0, 6));
                            println!("Failed to parse token rule {}: {:#?}", self.peek(1).text(), e);
                        }
                    }
                },

                RuleID => {
                    match self.rule_spec() {
                        Ok(r) => rules.push(r),
                        Err(e) => {
                            println!("{:#?}", self.peek_n(0, 3));
                            println!("Failed to parse token rule {}: {:#?}", self.peek(1).text(), e);
                        }
                    }
                },

                EOF => {
                    break;
                }

                t => {
                    return Err(ParserErr::NoTokenMatched { expected: vec![TokenID, RuleID, EOF], got: t })
                }
            }
        }
        
        Ok(ANTLRAst::new(rules, token_rules))
    }

    pub fn rule_spec(&mut self) -> Result<Rule, ParserErr> {
        let rule_name = self.match_token(RuleID)?.text().clone();
        
        self.match_token(Colon)?;

        let alts = self.alt_list()?;

        self.match_token(Semi)?;

        Ok(Rule::new(rule_name, alts))
    }

    pub fn token_rule_spec(&mut self) -> Result<TokenRule, ParserErr> {
        let is_fragment = self.peek_type(1) == Fragment;
        if is_fragment {
            self.consume(1);
        }

        let rule_name = self.match_token(TokenID)?.text().clone();
        
        self.match_token(Colon)?;

        let alts = self.alt_list()?;

        self.match_token(Semi)?;

        Ok(TokenRule::new(is_fragment, rule_name, alts))
    }

    pub fn block(&mut self) -> Result<Block, ParserErr> {
        self.match_token(LParen)?;
        let alts = self.alt_list()?;
        self.match_token(RParen)?;

        Ok(Block(alts))
    }

    pub fn element(&mut self) -> Result<Element, ParserErr> {
        // Elements are atom or block  and then a suffix?
        match self.peek_type(1) {
            StringLit => {
                let atom = self.atom()?;
                let suffix = self.ebnf_suffix().ok();
                Ok(Element::Atom { atom, suffix })
            },
 
            TokenID | RuleID => {
                let atom = self.atom()?;
                let suffix = self.ebnf_suffix().ok();
                Ok(Element::Atom { atom, suffix })
            },

            Not | Charset => {
                let token = self.peek(1);
                Ok(if token.token_type() == Not {
                    self.consume(1);
                    let token = self.match_token(Charset)?;

                    let suffix = self.ebnf_suffix().ok();
                    Element::Set { inverted: true, set: HashSet::from_iter(token.text().chars().map(|c| c as usize)), suffix }
                } else {
                    self.consume(1);

                    let suffix = self.ebnf_suffix().ok();
                    Element::Set { inverted: false, set: HashSet::from_iter(token.text().chars().map(|c| c as usize)), suffix}
                })
            }

            LParen => {
                let block = self.block()?;
                let suffix = self.ebnf_suffix().ok();

                Ok(Element::Block { block, suffix })
            }

            t => {
                Err(ParserErr::NoTokenMatched { expected: vec![StringLit, TokenID, RuleID, LParen], got: t })
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
            let label = if self.match_token(Pound).is_ok() {
                Some(self.match_any_token(vec![RuleID, TokenID])?.text())
            } else {
                None
            };
            
            let channel = if self.match_token(Arrow).is_ok() {
                // Match only channel right now, I'll do the rest later
                self.match_token(RuleID)?;
                self.match_token(LParen)?;
                let id = self.match_any_token(vec![RuleID, TokenID])?.text();
                self.match_token(RParen)?;

                Some(id)
            } else {
                None
            };

            return Ok(Some(Alt::new(label, elements, channel)))
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

        while let Ok(_) = self.match_token(OR) {
            match self.alt()? {
                Some(a) => alts.push(a),
                None => optional = true,
            }
        };
        
        Ok(AltList::new(optional, alts))
    }

    pub fn atom(&mut self) -> Result<Atom, ParserErr> {
        let token = self.match_any_token(vec![StringLit, RuleID, TokenID])?;

        match token.token_type() {
            StringLit => Ok(Atom::StringLit(token.text().clone())),
            TokenID | RuleID => Ok(Atom::ID(token.text().clone())),
            _ => unreachable!()
        }
    }

    pub fn ebnf_suffix(&mut self) -> Result<EBNFSuffix, ParserErr> {
        let token =  self.match_any_token(vec![Question, Star, Plus])?;

        match token.token_type() {
            Question => Ok(EBNFSuffix::Optional),
            Star => {
                if self.match_token(Question).is_ok() {
                    Ok(EBNFSuffix::StarOptional)
                } else {
                    Ok(EBNFSuffix::Star)
                }
            },
            Plus => {
                if self.match_token(Question).is_ok() {
                    Ok(EBNFSuffix::PlusOptional)
                } else {
                    Ok(EBNFSuffix::Plus)
                }
            }
            _ => unreachable!()
        }
    }
}