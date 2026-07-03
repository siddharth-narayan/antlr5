use std::marker::PhantomData;

use crate::antlr::{ANTLRAst, ANTLRTokenType::{self, ID, LParen, StringLit}, Parser, ParserErr, parse::{alternative::{Alt, AltList}, ebnf::EBNFSuffix, rules::{Atom, Block, Element, Rule}}};

impl Parser {
    pub fn grammar_spec(&mut self) -> Result<ANTLRAst, ParserErr> {
        let mut rules = Vec::new();

        while let Ok(rule) = self.rule_spec() {
            rules.push(rule)
        }
        
        Ok(ANTLRAst::new(rules))
    }

    pub fn rule_spec(&mut self) -> Result<Rule, ParserErr> {
        let rule_name = self.match_token(ANTLRTokenType::ID)?.text().clone();
        
        self.match_token(ANTLRTokenType::Colon)?;

        let alts = self.alt_list()?;

        self.match_token(ANTLRTokenType::Semi)?;

        Ok(Rule::new(rule_name, alts))
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
            StringLit => {
                let atom = self.atom()?;
                let suffix = self.ebnf_suffix().ok();
                Ok(Element::Atom { atom, suffix })
            },

            ID => {
                let atom = self.atom()?;
                let suffix = self.ebnf_suffix().ok();
                Ok(Element::Atom { atom, suffix })
            },

            LParen => {
                let block = self.block()?;
                let suffix = self.ebnf_suffix().ok();

                Ok(Element::Block { block, suffix })
            }

            t => {
                Err(ParserErr::NoTokenMatched { expected: vec![StringLit, ID, LParen], got: t })
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
                Some(self.match_token(ID)?.text())
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
        let token = self.match_any_token(vec![ANTLRTokenType::StringLit, ANTLRTokenType::ID])?;

        match token.token_type() {
            ANTLRTokenType::StringLit => Ok(Atom::StringLit(token.text().clone())),
            ANTLRTokenType::ID => Ok(Atom::ID(token.text().clone())),
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