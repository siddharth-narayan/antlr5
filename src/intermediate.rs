use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{analysis::SymbolTable, ast::{ANTLRAst, Alt, Atom, EBNFSuffix, Element, Rule, TokenRule}};

struct AntlrIR {
    rules: Vec<RuleIR>,
    token_rules: Vec<TokenRuleIR>,

    symbol_table: SymbolTable
}

impl AntlrIR {
    pub fn new(ast: ANTLRAst) -> AntlrIR {
        let symbol_table = SymbolTable::new(ast);

        let rules = symbol_table.rules().iter().map(|r| RuleIR::new(r, &symbol_table).unwrap()).collect();
        let token_rules = symbol_table.token_rules().iter().map(|r| TokenRuleIR::new(r, &symbol_table).unwrap()).collect();
        
        AntlrIR {
            rules,
            token_rules,
            symbol_table
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleIR {
    // modifiers: PhantomData<()>,
    // actions: PhantomData<()>,
    // return_val: PhantomData<()>,
    // throws_val: PhantomData<()>,
    // throws_spec: PhantomData<()>,
    // locals: PhantomData<()>,
    // prequel: PhantomData<()>,

    name: String,
    optional: bool,
    alts: Vec<AltIR>
}

impl RuleIR {
    pub fn new(rule: &Rule, table: &SymbolTable) -> Result<RuleIR, &'static str> {
        let name = rule.name().clone();
        let optional = rule.alt_list().optional();
        let mut alts = Vec::new();

        if rule.alt_list().alts().len() == 0 {
            return Err("There are no nonempty alts");
        };
        

        for alt in rule.alts() {
            alts.push(AltIR::new(alt, table)?);
           
        }

        return Ok(RuleIR {
            name: rule.name().clone(),
            optional,
            alts
        })

    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alts(&self) -> &Vec<AltIR> {
        &self.alts
    }


}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRuleIR {
    is_fragment: bool,
    name: String,
    optional: bool,
    alts: Vec<TokenAltIR>
}

impl TokenRuleIR {
    pub fn new(rule: &TokenRule, symbols: &SymbolTable) -> Result<TokenRuleIR, &'static str> {
        let name = rule.name().clone();
        let is_fragment = rule.is_fragment();
        let optional = rule.alt_list().optional();
        let mut alts = Vec::new();

        if rule.alt_list().alts().len() == 0 {
            return Err("There are no nonempty alts");
        };
        

        for alt in rule.alts() {
            alts.push(TokenAltIR::new(alt, symbols)?);
        }

        return Ok(TokenRuleIR {
            is_fragment,
            name: rule.name().clone(),
            optional,
            alts: alts
        })
    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alts(&self) -> &Vec<TokenAltIR> {
        &self.alts
    }
}




















#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct AltIR {
    label: Option<String>,
    options: PhantomData<()>,
    elements: Vec<ElementIR>,
    channel: Option<String>
}

// So bad it might as well be AI generated
impl AltIR {
    pub fn new(alt: &Alt, table: &SymbolTable) -> Result<AltIR, &'static str> {
        let label = alt.label().cloned();
        let channel = alt.channel().cloned();
        let mut elements = Vec::new();

        for element in alt.elements() {
            let suffix = element.suffix();
            
            let element = match element {
                Element::Atom { atom, suffix } => {
                    let atom = match atom {
                        Atom::ID(n) => {
                            if let Some(id) = table.get_rule_id(&n) {
                                AtomIR::RuleID(id)
                            } else if let Some(id) = table.get_token_id(&n) {
                                AtomIR::RuleID(id)
                            } else {
                                return Err("No rule id found");
                            }
                        },
                        Atom::StringLit(n) => {
                            AtomIR::TokenID(table.get_strlit_id(&n).expect("Strlit's should all be processed"))
                        }
                    };

                    // Move optional rules into their suffix to make generation easier
                    // table.get
                    ElementIR::Atom { atom, suffix: *suffix }
                },
                Element::Block { block, suffix } => {
                    let optional = block.0.optional();
                    let mut alts = Vec::new();
                    
                    for alt in block.0.alts() {
                        alts.push(AltIR::new(alt, table)?);
                    };

                    ElementIR::Block { block: BlockIR(alts), suffix: *suffix }
                },
                Element::Set { inverted, set, suffix } => {
                    return Err("Parser rules cannot contain lexer sets")
                }

            };

            elements.push(element);
        };

        Ok(AltIR { label, options: PhantomData, elements, channel })
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn elements(&self) -> &Vec<ElementIR> {
        &self.elements
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct TokenAltIR {
    label: Option<String>,
    options: PhantomData<()>,
    elements: Vec<ElementIR>,
    channel: Option<String>
}

impl TokenAltIR {
    pub fn new(alt: &Alt, table: &SymbolTable) -> Result<TokenAltIR, &'static str> {
        let label = alt.label().cloned();
        let channel = alt.channel().cloned();
        let mut elements = Vec::new();

        for element in alt.elements() {
            let suffix = element.suffix();
            
            let element = match element {
                Element::Atom { atom, suffix } => {
                    let atom = match atom {
                        Atom::ID(n) => {
                            if let Some(id) = table.get_rule_id(&n) {
                                AtomIR::RuleID(id)
                            } else if let Some(id) = table.get_token_id(&n) {
                                AtomIR::RuleID(id)
                            } else {
                                return Err("No rule id found");
                            }
                        },
                        Atom::StringLit(n) => {
                            AtomIR::TokenID(table.get_strlit_id(&n).expect("Strlit's should all be processed"))
                        }
                    };

                    // Move optional rules into their suffix to make generation easier
                    // table.get
                    ElementIR::Atom { atom, suffix: *suffix }
                },
                Element::Block { block, suffix } => {
                    let optional = block.0.optional();
                    let mut alts = Vec::new();
                    
                    for alt in block.0.alts() {
                        alts.push(AltIR::new(alt, table)?);
                    };

                    ElementIR::Block { block: BlockIR(alts), suffix: *suffix }
                },
                Element::Set { inverted, set, suffix } => {
                    return Err("Parser rules cannot contain lexer sets")
                }

            };

            elements.push(element);
        };

        Ok(TokenAltIR { label, options: PhantomData, elements, channel })
    }

    pub fn label(&self) -> Option<&String> {
        self.label.as_ref()
    }

    pub fn elements(&self) -> &Vec<ElementIR> {
        &self.elements
    }
}



// Should Element really have PartialEq/Eq derived?
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ElementIR {
    Atom {
        atom: AtomIR,
        suffix: Option<EBNFSuffix>
    },
    Block {
        block: BlockIR,
        suffix: Option<EBNFSuffix>
    },
    // EBNF(EBNF)
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum AtomIR {
    TokenID(usize),
    RuleID(usize)
}

impl ElementIR {
    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            ElementIR::Atom { suffix, .. } |
            ElementIR::Block { suffix, .. } => *suffix
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum TokenElementIR {
    Atom {
        atom: usize,
        suffix: Option<EBNFSuffix>
    },
    Block {
        block: BlockIR,
        suffix: Option<EBNFSuffix>
    },
    // EBNF(EBNF)
}

impl TokenElementIR {
    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            TokenElementIR::Atom { suffix, .. } |
            TokenElementIR::Block { suffix, .. } => *suffix
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct BlockIR(pub Vec<AltIR>);

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct TokenBlockIR(Vec<TokenAltIR>);


