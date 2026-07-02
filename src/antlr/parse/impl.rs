use crate::antlr::{ANTLRTokenType, Parser, ParserErr, parse::{alternative::Alt, ebnf::EBNFSuffix, rules::Rule}};

impl Parser {
    pub fn rule_spec(&mut self) -> Result<Rule, ParserErr> {
        let rule_name = self.match_token(ANTLRTokenType::ID)?.text().clone();
        let alts = Vec::new();
        
        self.match_token(ANTLRTokenType::Colon)?;

        println!("{}", rule_name);


        Ok(Rule::new(rule_name, alts))
    }

    pub fn alt(&mut self) -> Result<Alt, ParserErr> {
        match self.peek_token_type(1) {
            None
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