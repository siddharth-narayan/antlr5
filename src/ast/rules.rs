use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{analysis::{AnalysisErr, SymbolTable}, ast::{alternative::{Alt, AltList}, ebnf::EBNFSuffix}, codegen::{ATNFragment, State, StateRef, Transition}};

pub struct GrammarSpec {
    rule: Vec<Rule>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRule {
    is_fragment: bool,
    name: String,
    alt_list: AltList
}

impl TokenRule {
    pub fn new(is_fragment: bool, name: String, alt_list: AltList) -> TokenRule {
        TokenRule {
            is_fragment,
            name,
            alt_list
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alts(&self) -> &Vec<Alt> {
        &self.alt_list.alts()
    }

    pub fn codegen(&self, table: &SymbolTable) -> Result<ATNFragment, AnalysisErr> {
        self.alt_list.codegen(table)
    }

    pub fn symbols(&self, table: &mut SymbolTable) -> Result<(), AnalysisErr> {
        table.insert_token_rule(self.name.clone())?;

        self.alt_list.symbols(table)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    // modifiers: PhantomData<()>,
    // actions: PhantomData<()>,
    // return_val: PhantomData<()>,
    // throws_val: PhantomData<()>,
    // throws_spec: PhantomData<()>,
    // locals: PhantomData<()>,
    // prequel: PhantomData<()>,

    name: String,
    alt_list: AltList
}

impl Rule {
    pub fn new(name: String, alt_list: AltList) -> Rule {
        Rule {
            name,
            alt_list
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn alts(&self) -> &Vec<Alt> {
        &self.alt_list.alts()
    }

    pub fn codegen(&self, table: &SymbolTable) -> Result<ATNFragment, AnalysisErr> {
        self.alt_list.codegen(table)
    }

    pub fn symbols(&self, table: &mut SymbolTable) -> Result<(), AnalysisErr> {
        table.insert_rule(self.name.clone())?;

        self.alt_list.symbols(table)?;

        Ok(())
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Element {
    Atom {
        atom: Atom,
        suffix: Option<EBNFSuffix>
    },
    Block {
        block: Block,
        suffix: Option<EBNFSuffix>
    },
    Set {
        inverted: bool,
        set: HashSet<usize>,
        suffix: Option<EBNFSuffix>
    }
    // EBNF(EBNF)
}

impl Element {
    pub fn suffix(&self) -> Option<EBNFSuffix> {
        match self {
            Element::Atom { suffix, .. } |
            Element::Block { suffix, .. } |
            Element::Set { suffix, .. } => *suffix
        }
    }

    pub fn codegen(&self, table: &SymbolTable) -> Result<ATNFragment, AnalysisErr> {
        match self {
            Self::Atom { atom, .. } => {
                atom.codegen(table)
            },

            Self::Block { block, .. } => {
                block.codegen(table)
            },

            Self::Set { inverted, set, suffix } => {
                todo!()
            }
        }
    }

    pub fn symbols(&self, table: &mut SymbolTable) -> Result<(), AnalysisErr> {
        match self {
            Self::Atom { atom, .. } => {
                atom.symbols(table)
            },

            Self::Block { block, .. } => {
                block.symbols(table)
            },
            
            _ => Ok(())
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Block(pub AltList);

impl Block {
    pub fn symbols(&self, table: &mut SymbolTable) -> Result<(), AnalysisErr> {
        self.0.symbols(table)?;

        Ok(())
    }

    pub fn codegen(&self, table: &SymbolTable) -> Result<ATNFragment, AnalysisErr> {
        self.0.codegen(table)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum Atom {
    StringLit(String),
    ID(String)
}

impl Atom {
    pub fn symbols(&self, table: &mut SymbolTable) -> Result<(), AnalysisErr> {
        match self {
            Atom::StringLit(s) => table.insert_token(s.clone()),
            Atom::ID(_) => () // IDs will be checked at codegen time for validity
        }

        Ok(())
    }

    pub fn codegen(&self, table: &SymbolTable) -> Result<ATNFragment, AnalysisErr> {
        let mut fragment = ATNFragment::new();

        match self {
            Atom::StringLit(s) => {
                let id = table.get_token_id(s.clone()).ok_or(AnalysisErr::Undefined { name: s.clone() })?;

                // States literally don't hold useful info
                let state = State::new();
                fragment.push_state(state);
                fragment.push_transition(Transition::Atom { source: StateRef(0), target: StateRef(1), input: id });
            },
            Atom::ID(s) => {
                // let rule_id = table.get_rule_id(name).ok_or(AnalysisErr::Undefined { name: s.clone() })?;
            }
        }

        Ok(fragment)
    }
}