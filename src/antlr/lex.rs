use crate::antlr::{
    lex::ANTLRTokenType::*,
    lex::LexerErr::*,
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
    ParserToken,
    LexerToken,
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
            ',' => Ok(Comma),
            ';' => Ok(Semi),

            '(' => Ok(LParen),
            ')' => Ok(RParen),
            '{' => Ok(LBrace),
            '}' => Ok(RBrace),
            '>' => Ok(GT),
            '<' => Ok(LT),
            '=' => Ok(Assign),
            '|' => Ok(OR),
            '$' => Ok(Dollar),
            '?' => Ok(Question),
            '*' => Ok(Star),
            '@' => Ok(At),
            '#' => Ok(Pound),
            '~' => Ok(Not),

            '0' => Ok(Int),
            '1'..='9' => loop {
                match self.peek(1) {
                    Some(n) => {
                        if !('0'..'9').contains(&n) {
                            break Ok(Int);
                        } else {
                            self.consume(1);
                            current_text.push(n);
                        }
                    }

                    None => break Ok(Int),
                }
            },

            ':' => match self.peek(1) {
                Some(':') => {
                    self.consume(1);
                    Ok(DoubleColon)
                }
                _ => Ok(Colon),
            },

            '+' => match self.peek(1) {
                Some('=') => {
                    self.consume(1);
                    Ok(PlusAssign)
                }
                _ => Ok(Plus),
            },

            '.' => match self.peek(1) {
                Some('.') => {
                    self.consume(1);
                    Ok(Range)
                }
                _ => Ok(Dot),
            },

            '-' => match self.peek(1) {
                Some('>') => {
                    self.consume(1);
                    Ok(Arrow)
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
                    _ => break Ok(WS),
                }
            },

            'p' => {
                if self.match_keyword("arser").is_ok() {
                    Ok(ParserToken)
                } else if self.match_keyword("rotected").is_ok() {
                    Ok(Protected)
                } else if self.match_keyword("ublic").is_ok() {
                    Ok(Public)
                } else if self.match_keyword("rivate").is_ok() {
                    Ok(Private)
                } else {
                    Err(UnmatchedKeyword)
                }
            }

            'o' => {
                if self.match_keyword("ptions").is_ok() {
                    Ok(Options)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            'f' =>  {
                if self.match_keyword("inally").is_ok() {
                    Ok(Finally)
                } else if self.match_keyword("ragment").is_ok() {
                    Ok(Fragment)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            't' => {
                if self.match_keyword("hrows").is_ok() {
                    Ok(Throws)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            // Also catch
            'c' => {
                if self.match_keyword("atch").is_ok() {
                    Ok(Catch)
                } else if self.match_keyword("hannels").is_ok() {
                    Ok(Channels)
                } else {
                    Err(UnmatchedKeyword)
                }
            }

            'r' => {
                if self.match_keyword("ptions").is_ok() {
                    Ok(Options)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            'g' => {
                if self.match_keyword("rammar").is_ok() {
                    Ok(Grammar)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            'i' => {
                if self.match_keyword("mport").is_ok() {
                    Ok(Import)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            'm' => {
                if self.match_keyword("ode").is_ok() {
                    Ok(Mode)
                } else {
                    Err(UnmatchedKeyword)
                }
            },

            'l' => {
                if self.match_keyword("exer").is_ok() {
                    Ok(LexerToken)
                } else if self.match_keyword("ocals").is_ok() {
                    Ok(Locals)
                } else {
                    Err(UnmatchedKeyword)
                }
            }

            '/' => {
                match self.peek(1) {
                    Some('/') => {
                        loop {
                            if let Some('\n') = self.next() {
                                break Ok(Comment)
                            }
                        } 
                    },

                    Some('*') => {
                        loop {
                            if let Some('*') = self.next() {
                                if let Some('/') = self.peek(1) {
                                    self.consume(1);
                                    break Ok(CommentBlock)
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
                                break Ok(StringLit);
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
                                current_text.push('\u{000C}');
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
                                break Ok(Charset);
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
                                current_text.push('\u{000C}');
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
                    token_type: if is_token { TokenID } else { RuleID },
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
