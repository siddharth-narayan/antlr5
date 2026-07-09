use crate::antlr::{
    LexerErr::{UnkownCharacter, UnmatchedKeyword},
    lex::LexerErr::UnexpectedCharacter,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ANTLRTokenType {
    EOF,
    
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
    RangeDash,
    Colon,
    DoubleColon,
    Star,
    Int,
    Dot,
    Range,
    Plus,
    PlusAssign,
    Comment,
    CommentBlock,

    RuleID,
    TokenID,

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
    Charset
}

#[derive(Clone, Debug)]
pub struct ANTLRToken {
    token_type: ANTLRTokenType,
    text: String,
}

impl ANTLRToken {
    pub fn new(token_type: ANTLRTokenType, text: String) -> ANTLRToken {
        ANTLRToken { token_type, text }
    }

    pub fn token_type(&self) -> ANTLRTokenType {
        self.token_type
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }
}

#[derive(Debug)]
pub enum LexerErr {
    EOF,
    UnexpectedCharacter { expected: char, got: Option<char> },
    // A character that is not recognized by the lexer
    UnkownCharacter(char),
    UnmatchedKeyword,
}

pub struct Lexer {
    input: Vec<char>,
    head: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            head: 0,
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

    pub fn next_token(&mut self) -> Result<ANTLRToken, LexerErr> {
        let mut current_text = Vec::new();

        let character = match self.next() {
            Some(c) => c,
            None => {
                return Ok(ANTLRToken {
                    token_type: ANTLRTokenType::EOF,
                    text: current_text.iter().collect()
                })
            }
        };

        let token = match character  {
            ',' => Ok(ANTLRTokenType::Comma),
            ';' => Ok(ANTLRTokenType::Semi),

            '(' => Ok(ANTLRTokenType::LParen),
            ')' => Ok(ANTLRTokenType::RParen),
            '{' => Ok(ANTLRTokenType::LBrace),
            '}' => Ok(ANTLRTokenType::RBrace),
            '>' => Ok(ANTLRTokenType::GT),
            '<' => Ok(ANTLRTokenType::LT),
            '=' => Ok(ANTLRTokenType::Assign),
            '|' => Ok(ANTLRTokenType::OR),
            '$' => Ok(ANTLRTokenType::Dollar),
            '?' => Ok(ANTLRTokenType::Question),
            '*' => Ok(ANTLRTokenType::Star),
            '@' => Ok(ANTLRTokenType::At),
            '#' => Ok(ANTLRTokenType::Pound),
            '~' => Ok(ANTLRTokenType::Not),

            '0' => Ok(ANTLRTokenType::Int),
            '1'..='9' => loop {
                match self.peek(1) {
                    Some(n) => {
                        if !('0'..'9').contains(&n) {
                            break Ok(ANTLRTokenType::Int);
                        } else {
                            self.consume(1);
                            current_text.push(n);
                        }
                    }

                    None => break Ok(ANTLRTokenType::Int),
                }
            },

            ':' => match self.peek(1) {
                Some(':') => {
                    self.consume(1);
                    Ok(ANTLRTokenType::DoubleColon)
                }
                _ => Ok(ANTLRTokenType::Colon),
            },

            '+' => match self.peek(1) {
                Some('=') => {
                    self.consume(1);
                    Ok(ANTLRTokenType::PlusAssign)
                }
                _ => Ok(ANTLRTokenType::Plus),
            },

            '.' => match self.peek(1) {
                Some('.') => {
                    self.consume(1);
                    Ok(ANTLRTokenType::Range)
                }
                _ => Ok(ANTLRTokenType::Dot),
            },

            '-' => match self.peek(1) {
                Some('>') => {
                    self.consume(1);
                    Ok(ANTLRTokenType::Arrow)
                },

                Some(n) => {
                    Err(UnexpectedCharacter {
                        expected: '>',
                        got: Some(n),
                    })
                },
                
                None => Err(LexerErr::EOF),
            },

            ' ' | '\t' | '\r' | '\n' => loop {
                match self.peek(1) {
                    Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
                        self.consume(1);
                    }
                    _ => break Ok(ANTLRTokenType::WS),
                }
            },

            'p' => {
                if self.match_keyword("arser").is_ok() {
                    Ok(ANTLRTokenType::Parser)
                } else if self.match_keyword("rotected").is_ok() {
                    Ok(ANTLRTokenType::Protected)
                } else if self.match_keyword("ublic").is_ok() {
                    Ok(ANTLRTokenType::Public)
                } else if self.match_keyword("rivate").is_ok() {
                    Ok(ANTLRTokenType::Private)
                } else {
                    Err(UnmatchedKeyword)
                }
            }

            'o' => {
                if self.match_keyword("ptions").is_ok() {
                    Ok(ANTLRTokenType::Options)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            'f' =>  {
                if self.match_keyword("inally").is_ok() {
                    Ok(ANTLRTokenType::Finally)
                } else if self.match_keyword("ragment").is_ok() {
                    Ok(ANTLRTokenType::Fragment)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            't' => {
                if self.match_keyword("hrows").is_ok() {
                    Ok(ANTLRTokenType::Throws)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            // Also catch
            'c' => {
                if self.match_keyword("atch").is_ok() {
                    Ok(ANTLRTokenType::Catch)
                } else if self.match_keyword("hannels").is_ok() {
                    Ok(ANTLRTokenType::Channels)
                } else {
                    Err(UnmatchedKeyword)
                }
            }

            'r' => {
                if self.match_keyword("ptions").is_ok() {
                    Ok(ANTLRTokenType::Options)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            'g' => {
                if self.match_keyword("ptions").is_ok() {
                    Ok(ANTLRTokenType::Options)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            'i' => {
                if self.match_keyword("mport").is_ok() {
                    Ok(ANTLRTokenType::Import)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            'm' => {
                if self.match_keyword("ode").is_ok() {
                    Ok(ANTLRTokenType::Mode)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            'l' => {
                if self.match_keyword("exer").is_ok() {
                    Ok(ANTLRTokenType::Lexer)
                } else if self.match_keyword("ocals").is_ok() {
                    Ok(ANTLRTokenType::Locals)
                } else {
                    Err(UnmatchedKeyword)
                }
            }

            '/' => {
                match self.peek(1) {
                    Some('/') => {
                        loop {
                            if let Some('\n') = self.next() {
                                break Ok(ANTLRTokenType::Comment)
                            }
                        } 
                    },

                    Some('*') => {
                        loop {
                            if let Some('*') = self.next() {
                                if let Some('/') = self.peek(1) {
                                    self.consume(1);
                                    break Ok(ANTLRTokenType::CommentBlock)
                                }
                            }
                        }
                    }

                    n => {
                        Err(LexerErr::UnexpectedCharacter { expected: '/', got: n })
                    }
                    
                }
            },

            '\'' => {
                let mut esc = false;
                loop {
                    match self.next() {
                        None => {
                            break Err(LexerErr::EOF);
                        },

                        Some('\\') => {
                            if esc {
                                current_text.push('\\');
                                esc = false;
                            } else {
                                esc = true
                            }
                        },

                        Some('\'') => {
                            if esc {
                                current_text.push('\'');
                                esc = false;
                            } else {
                                break Ok(ANTLRTokenType::StringLit);
                            }
                        },

                        Some('n') => {
                            if esc {
                                current_text.push('\n');
                                esc = false;
                            } else {
                                current_text.push('n')
                            }
                        },

                        Some('t') => {
                            if esc {
                                current_text.push('\t');
                                esc = false;
                            } else {
                                current_text.push('t')
                            }
                        },

                        Some('r') => {
                            if esc {
                                current_text.push('\r');
                                esc = false;
                            } else {
                                current_text.push('r')
                            }
                        },
                        
                        Some('f') => {
                            if esc {
                                // current_text.push('');
                                esc = false;
                            } else {
                                current_text.push('f')
                            }
                        },
                        
                        Some('u') => {
                            if esc {
                                todo!();
                                esc = false;
                            } else {
                                current_text.push('u')
                            }
                        },

                        Some(n) => {
                            current_text.push(n);
                        }
                    }
                }
            }
            // The exact same as a string literal but the text of the token is meant as a Vec of all the characters
            '[' => {
                let mut esc = false;
                loop {
                    match self.next() {
                        None => {
                            break Err(LexerErr::EOF);
                        },

                        Some('\\') => {
                            if esc {
                                current_text.push('\\');
                                esc = false;
                            } else {
                                esc = true
                            }
                        },

                        Some(']') => {
                            if esc {
                                current_text.push(']');
                                esc = false;
                            } else {
                                break Ok(ANTLRTokenType::Charset);
                            }
                        },

                        Some('-') => {
                            if esc {
                                current_text.push('-');
                                esc = false;
                            } else {
                                println!("The character \"-\" is only available with a preceding and following character, as part of a range, or you might want to escape it");
                                break Err(LexerErr::UnkownCharacter('-'))
                            }
                        },

                        Some('n') => {
                            if esc {
                                current_text.push('\n');
                                esc = false;
                            } else {
                                current_text.push('n')
                            }
                        },

                        Some('t') => {
                            if esc {
                                current_text.push('\t');
                                esc = false;
                            } else {
                                current_text.push('t')
                            }
                        },

                        Some('r') => {
                            if esc {
                                current_text.push('\r');
                                esc = false;
                            } else {
                                current_text.push('r')
                            }
                        },
                        
                        Some('f') => {
                            if esc {
                                // current_text.push('');
                                esc = false;
                            } else {
                                current_text.push('f')
                            }
                        },
                        
                        Some('u') => {
                            if esc {
                                todo!();
                                esc = false;
                            } else {
                                current_text.push('u')
                            }
                        },

                        Some(n) => {
                            current_text.push(n);
                        }
                    }


                    if let Some('-') = self.peek(1) && !esc {
                        let until = match self.peek(2) {
                            None => break Err(LexerErr::EOF),
                            Some('\\') => todo!("I haven't done implemented escaped ranges yet"),
                            Some(n) => n
                        };

                        self.consume(2);
                        let from = current_text.pop().unwrap();
                        for c in from..=until {
                            current_text.push(c);
                        }
                    }
                }
            }
            n => Err(UnkownCharacter(n)),
        };

        // Fallback to matching ID
        match token {
            Ok(t) => Ok( ANTLRToken { token_type: t, text: current_text.iter().collect() }),
            Err(e) => {
                let mut is_token = false;

                match character {
                    'a'..='z' => {
                        current_text.push(character);
                    },

                    'A'..='Z' => {
                        is_token = true;
                        current_text.push(character);
                    },

                    _ => {
                        return Err(e)
                    }
                };

                loop {
                    let c = match self.peek(1) {
                        Some(c) => c,
                        None => break 
                    };

                    match c {
                        'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
                            current_text.push(c);
                            self.consume(1)
                        }

                        _ => break
                    }
                }

                Ok(ANTLRToken {
                    token_type: if is_token { ANTLRTokenType::TokenID } else { ANTLRTokenType::RuleID },
                    text: current_text.iter().collect()
                })
            }
        }
    }

    pub fn match_keyword(&mut self, string: &str) -> Result<(), Option<usize>> {
        let chars = string.chars();

        for (index, c) in chars.enumerate() {
            if c != self.peek(index + 1).ok_or(index)? {
                return Err(Some(index));
            }
        }

        self.consume(string.chars().count());

        // Check there's whitespace after
        match self.peek(1) {
            Some('A'..='Z') | Some('a'..='z') => Err(None),
            _ => Ok(())
        }
    }
}
