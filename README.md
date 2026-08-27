This grammar
```antlr
// The main entry point for parsing a v4 grammar.
grammarSpec
    : grammarDecl prequelConstruct* rules modeSpec* EOF
    ;

grammarDecl
    : grammarType identifier SEMI
    ;

grammarType
    : LEXER GRAMMAR
    | PARSER GRAMMAR
    | GRAMMAR
```

Produces the following tokens
```
ANTLRToken {
    token_type: RuleID,
    text: "grammarSpec",
}
ANTLRToken {
    token_type: Colon,
    text: "",
}
ANTLRToken {
    token_type: RuleID,
    text: "grammarDecl",
}
ANTLRToken {
    token_type: RuleID,
    text: "prequelConstruct",
}
ANTLRToken {
    token_type: Star,
    text: "",
}
ANTLRToken {
    token_type: RuleID,
    text: "rules",
}
ANTLRToken {
    token_type: RuleID,
    text: "mSpec",
}
ANTLRToken {
    token_type: Star,
    text: "",
}
ANTLRToken {
    token_type: TokenID,
    text: "EOF",
}
ANTLRToken {
    token_type: Semi,
    text: "",
}
ANTLRToken {
    token_type: RuleID,
    text: "grammarDecl",
}
ANTLRToken {
    token_type: Colon,
    text: "",
}
ANTLRToken {
    token_type: RuleID,
    text: "grammarType",
}
ANTLRToken {
    token_type: RuleID,
    text: "identifier",
}
ANTLRToken {
    token_type: TokenID,
    text: "SEMI",
}
ANTLRToken {
    token_type: Semi,
    text: "",
}
ANTLRToken {
    token_type: RuleID,
    text: "grammarType",
}
ANTLRToken {
    token_type: Colon,
    text: "",
}
ANTLRToken {
    token_type: TokenID,
    text: "LEXER",
}
ANTLRToken {
    token_type: TokenID,
    text: "GRAMMAR",
}
ANTLRToken {
    token_type: OR,
    text: "",
}
ANTLRToken {
    token_type: TokenID,
    text: "PARSER",
}
ANTLRToken {
    token_type: TokenID,
    text: "GRAMMAR",
}
ANTLRToken {
    token_type: OR,
    text: "",
}
ANTLRToken {
    token_type: TokenID,
    text: "GRAMMAR",
}
ANTLRToken {
    token_type: Semi,
    text: "",
}
```
And the following parse tree
```
ANTLRAst {
    rules: [
        Rule {
            name: "grammarSpec",
            alt_list: AltList {
                optional: false,
                alts: [
                    Alt {
                        label: None,
                        options: PhantomData<()>,
                        elements: [
                            Atom {
                                atom: ID(
                                    "grammarDecl",
                                ),
                                suffix: None,
                            },
                            Atom {
                                atom: ID(
                                    "prequelConstruct",
                                ),
                                suffix: Some(
                                    Star,
                                ),
                            },
                            Atom {
                                atom: ID(
                                    "rules",
                                ),
                                suffix: None,
                            },
                            Atom {
                                atom: ID(
                                    "mSpec",
                                ),
                                suffix: Some(
                                    Star,
                                ),
                            },
                            Atom {
                                atom: ID(
                                    "EOF",
                                ),
                                suffix: None,
                            },
                        ],
                        channel: None,
                    },
                ],
            },
        },
        Rule {
            name: "grammarDecl",
            alt_list: AltList {
                optional: false,
                alts: [
                    Alt {
                        label: None,
                        options: PhantomData<()>,
                        elements: [
                            Atom {
                                atom: ID(
                                    "grammarType",
                                ),
                                suffix: None,
                            },
                            Atom {
                                atom: ID(
                                    "identifier",
                                ),
                                suffix: None,
                            },
                            Atom {
                                atom: ID(
                                    "SEMI",
                                ),
                                suffix: None,
                            },
                        ],
                        channel: None,
                    },
                ],
            },
        },
        Rule {
            name: "grammarType",
            alt_list: AltList {
                optional: false,
                alts: [
                    Alt {
                        label: None,
                        options: PhantomData<()>,
                        elements: [
                            Atom {
                                atom: ID(
                                    "LEXER",
                                ),
                                suffix: None,
                            },
                            Atom {
                                atom: ID(
                                    "GRAMMAR",
                                ),
                                suffix: None,
                            },
                        ],
                        channel: None,
                    },
                    Alt {
                        label: None,
                        options: PhantomData<()>,
                        elements: [
                            Atom {
                                atom: ID(
                                    "PARSER",
                                ),
                                suffix: None,
                            },
                            Atom {
                                atom: ID(
                                    "GRAMMAR",
                                ),
                                suffix: None,
                            },
                        ],
                        channel: None,
                    },
                    Alt {
                        label: None,
                        options: PhantomData<()>,
                        elements: [
                            Atom {
                                atom: ID(
                                    "GRAMMAR",
                                ),
                                suffix: None,
                            },
                        ],
                        channel: None,
                    },
                ],
            },
        },
    ],
    token_rules: [],
}
SymbolTable {
    rules: {
        "grammarType": 2,
        "grammarDecl": 1,
        "grammarSpec": 0,
    },
    token_rules: {},
    tokens: {},
}
```


Differences from current ANTLR
- Charsets work differently. They can not be represented as ~('a' | 'b') etc. Represent this as ~[ab]. In addition, '-' must be escaped within a charset unless it's being used as a range.