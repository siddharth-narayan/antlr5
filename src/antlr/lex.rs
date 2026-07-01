use crate::antlr::{
    LexerErr::{UnkownCharacter, UnmatchedString},
    lex::LexerErr::UnexpectedCharacter,
};

#[derive(Debug)]
pub enum ANTLRTokenType {
    Comma,
    Semi,
    WS,

    LParen,
    RParen,
    LBrace,
    RBrace,
    GT,
    LT,
    Assign,
    OR,
    Dollar,
    Pound,
    At,
    Not,
    Question,
    Arrow,
    Colon,
    DoubleColon,
    Star,
    Int,
    Dot,
    Range,
    Plus,
    PlusAssign,

    Fragment,
    Parser,
    Lexer,
    Grammar,
    Options,
    Tokens,
    Channels,
    Import,
    Public,
    Protected,
    Private,
    Returns,
    Locals,
    Throws,
    Catch,
    Finally,
    Mode,
    StringLit,
}

pub struct ANTLRToken {
    token_type: ANTLRTokenType,
    text: String,
}

#[derive(Debug)]
pub enum LexerErr {
    EOF,
    UnexpectedCharacter { expected: char, got: Option<char> },
    // A character that is not recognized by the lexer
    UnkownCharacter(char),
    UnmatchedString,
}

pub struct Lexer {
    input: Vec<char>,
    head: usize,
    current_text: Vec<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            head: 0,
            current_text: Vec::new(),
        }
    }

    pub fn next(&mut self) -> Option<char> {
        let res = self.input.get(self.head);
        self.head += 1;

        res.copied()
    }

    pub fn consume(&mut self, amount: usize) {
        self.head += amount
    }

    pub fn peek(&self, by: usize) -> Option<char> {
        self.input.get(self.head + by - 1).copied()
    }

    pub fn peek_n(&self, by: usize, count: usize) -> Option<&[char]> {
        self.input
            .get((self.head + by - 1)..(self.head + by + count - 1))
    }

    pub fn next_token(&mut self) -> Result<ANTLRTokenType, LexerErr> {
        let token = match self.next().ok_or(LexerErr::EOF)? {
            ',' => ANTLRTokenType::Comma,
            ';' => ANTLRTokenType::Semi,

            '(' => ANTLRTokenType::LParen,
            ')' => ANTLRTokenType::RParen,
            '{' => ANTLRTokenType::LBrace,
            '}' => ANTLRTokenType::RBrace,

            '>' => ANTLRTokenType::GT,
            '<' => ANTLRTokenType::LT,
            '=' => ANTLRTokenType::Assign,
            '|' => ANTLRTokenType::OR,
            '$' => ANTLRTokenType::Dollar,
            '?' => ANTLRTokenType::Question,
            '*' => ANTLRTokenType::Star,
            '@' => ANTLRTokenType::At,
            '#' => ANTLRTokenType::Pound,
            '~' => ANTLRTokenType::Not,

            '0' => ANTLRTokenType::Int,
            '1'..='9' => loop {
                match self.peek(1) {
                    Some(n) => {
                        if !('0'..'9').contains(&n) {
                            break ANTLRTokenType::Int;
                        } else {
                            self.consume(1);
                            self.current_text.push(n);
                        }
                    }

                    None => break ANTLRTokenType::Int,
                }
            },

            ':' => match self.peek(1) {
                Some(':') => {
                    self.consume(1);
                    ANTLRTokenType::DoubleColon
                }
                _ => ANTLRTokenType::Colon,
            },

            '+' => match self.peek(1) {
                Some('=') => {
                    self.consume(1);
                    ANTLRTokenType::PlusAssign
                }
                _ => ANTLRTokenType::Plus,
            },

            '.' => match self.peek(1) {
                Some('.') => {
                    self.consume(1);
                    ANTLRTokenType::Range
                }
                _ => ANTLRTokenType::Dot,
            },

            '-' => match self.peek(1) {
                Some('>') => {
                    self.consume(1);
                    ANTLRTokenType::Arrow
                }

                Some(n) => {
                    return Err(UnexpectedCharacter {
                        expected: '>',
                        got: Some(n),
                    });
                }
                None => return Err(LexerErr::EOF),
            },

            ' ' | '\t' | '\r' | '\n' => loop {
                match self.peek(1) {
                    Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
                        self.consume(1);
                    }
                    _ => break ANTLRTokenType::WS,
                }
            },

            'p' => {
                if self.match_string("arser").is_ok() {
                    ANTLRTokenType::Parser
                } else if self.match_string("rotected").is_ok() {
                    ANTLRTokenType::Protected
                } else if self.match_string("ublic").is_ok() {
                    ANTLRTokenType::Public
                } else if self.match_string("rivate").is_ok() {
                    ANTLRTokenType::Private
                } else {
                    return Err(UnmatchedString);
                }
            }

            'o' => match self.match_string("ptions") {
                Ok(_) => ANTLRTokenType::Options,
                Err(n) => {
                    return Err(UnexpectedCharacter {
                        expected: "ptions".chars().nth(n).unwrap(),
                        got: self.peek(n + 1),
                    });
                }
            },

            'f' => match self.match_string("inally") {
                Ok(_) => ANTLRTokenType::Finally,
                Err(n) => {
                    return Err(UnexpectedCharacter {
                        expected: "inally".chars().nth(n).unwrap(),
                        got: self.peek(n + 1),
                    });
                }
            },

            't' => match self.match_string("hrows") {
                Ok(_) => ANTLRTokenType::Throws,
                Err(n) => {
                    return Err(UnexpectedCharacter {
                        expected: "hrows".chars().nth(n).unwrap(),
                        got: self.peek(n + 1),
                    });
                }
            },

            // Also catch
            'c' => {
                if self.match_string("atch").is_ok() {
                    ANTLRTokenType::Catch
                } else if self.match_string("hannels").is_ok() {
                    ANTLRTokenType::Channels
                } else {
                    return Err(UnmatchedString);
                }
            }

            'r' => match self.match_string("eturns") {
                Ok(_) => ANTLRTokenType::Returns,
                Err(n) => {
                    return Err(UnexpectedCharacter {
                        expected: "eturns".chars().nth(n).unwrap(),
                        got: self.peek(n + 1),
                    });
                }
            },

            'g' => match self.match_string("rammar") {
                Ok(_) => ANTLRTokenType::Grammar,
                Err(n) => {
                    return Err(UnexpectedCharacter {
                        expected: "rammar".chars().nth(n).unwrap(),
                        got: self.peek(n + 1),
                    });
                }
            },

            'i' => match self.match_string("mport") {
                Ok(_) => ANTLRTokenType::Import,
                Err(n) => {
                    return Err(UnexpectedCharacter {
                        expected: "mport".chars().nth(n).unwrap(),
                        got: self.peek(n + 1),
                    });
                }
            },

            'm' => match self.match_string("ode") {
                Ok(_) => ANTLRTokenType::Mode,
                Err(n) => {
                    return Err(UnexpectedCharacter {
                        expected: "ode".chars().nth(n).unwrap(),
                        got: self.peek(n + 1),
                    });
                }
            },

            'l' => {
                if self.match_string("exer").is_ok() {
                    ANTLRTokenType::Lexer
                } else if self.match_string("ocals").is_ok() {
                    ANTLRTokenType::Locals
                } else {
                    return Err(UnmatchedString);
                }
            }

            '\'' => {
                let mut esc = false;
                loop {
                    match self.next() {
                        None => {
                            return Err(LexerErr::EOF);
                        }

                        Some('\\') => esc = true,

                        Some('\'') => {
                            if esc {
                                self.current_text.push('\'')
                            } else {
                                break ANTLRTokenType::StringLit;
                            }
                        }

                        Some('n') => {
                            if esc {
                                self.current_text.push('\n')
                            } else {
                                self.current_text.push('n')
                            }
                        }

                        Some('t') => {
                            if esc {
                                self.current_text.push('\t')
                            } else {
                                self.current_text.push('t')
                            }
                        }

                        Some('r') => {
                            if esc {
                                self.current_text.push('\r')
                            } else {
                                self.current_text.push('r')
                            }
                        }

                        Some('u') => {
                            if esc {
                                todo!()
                            } else {
                                self.current_text.push('u')
                            }
                        }

                        Some(n) => {
                            self.current_text.push(n);
                        }
                    }
                }
            }

            n => return Err(UnkownCharacter(n)),
        };

        Ok(token)
    }

    pub fn match_string(&mut self, string: &str) -> Result<(), usize> {
        let chars = string.chars();

        for (index, c) in chars.enumerate() {
            if c != self.peek(index + 1).ok_or(index)? {
                return Err(index);
            }
        }

        self.consume(string.chars().count());

        Ok(())
    }
}
