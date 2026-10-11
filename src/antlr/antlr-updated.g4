grammar antlr;

// The main entry point for parsing a v4 grammar.
grammarSpec
    : grammarDecl rules EOF
    ;

grammarDecl
    : grammarType identifier Semi
    ;

grammarType
    : Lexer Grammar
    | Parser Grammar
    | Grammar
    ;

rules
    : ruleSpec*
    ;

ruleSpec
    : parserRuleSpec
    | lexerRuleSpec
    ;

parserRuleSpec
    : RuleID Colon ruleBlock Semi
    ;

ruleBlock
    : ruleAltList
    ;

ruleAltList
    : labeledAlt (OR labeledAlt)*
    ;

labeledAlt
    : alternative (Pound identifier)?
    ;

// --------------------
// Lexer rules

lexerRuleSpec
    : Fragment? TokenID Colon lexerRuleBlock Semi
    ;

lexerRuleBlock
    : lexerAltList
    ;

lexerAltList
    : lexerAlt (OR lexerAlt)*
    ;

lexerAlt
    : lexerElements
    |
    // explicitly allow empty alts
    ;

lexerElements
    : lexerElement+
    |
    ;

lexerElement
    : lexerAtom ebnfSuffix?
    | lexerBlock ebnfSuffix?
    ;

// but preds can be anywhere

lexerBlock
    : LParen lexerAltList RParen
    ;

// E.g., channel(HIDDEN), skip, more, mode(INSIDE), push(INSIDE), pop

// --------------------
// Rule Alts

altList
    : alternative (OR alternative)*
    ;

alternative
    : elementOptions? element+
    |
    // explicitly allow empty alts
    ;

element
    : labeledElement (ebnfSuffix |)
    | atom (ebnfSuffix |)
    | ebnf
    ;

labeledElement
    : identifier (Assign | PlusAssign) (atom | block)
    ;

// --------------------
// EBNF and blocks

ebnf
    : block blockSuffix?
    ;

blockSuffix
    : ebnfSuffix
    ;

ebnfSuffix
    : Question Question?
    | Star Question?
    | Plus Question?
    ;

lexerAtom
    : characterRange
    | terminalDef
    | notSet
    | wildcard
    ;

atom
    : terminalDef
    | ruleref
    | notSet
    | wildcard
    ;

wildcard
    : Dot elementOptions?
    ;

// --------------------
// Inverted element set
notSet
    : Not setElement
    | Not blockSet
    ;

blockSet
    : LParen setElement (OR setElement)* RParen
    ;

setElement
    : TokenID elementOptions?
    | StringLit elementOptions?
    | characterRange
    ;

// -------------
// Grammar Block
block
    : LParen altList RParen
    ;

// ----------------
// Parser rule ref
ruleref
    : RuleID elementOptions?
    ;

// ---------------
// Character Range
characterRange
    : StringLit Range StringLit
    ;

terminalDef
    : TokenID elementOptions?
    | StringLit elementOptions?
    ;

// Terminals may be adorned with certain options when
// reference in the grammar: TOK<,,,>
elementOptions
    : LT elementOption (Comma elementOption)* GT
    ;

elementOption
    : qualifiedIdentifier
    | identifier Assign (qualifiedIdentifier | StringLit | Int)
    ;

identifier
    : RuleID
    | TokenID
    ;

qualifiedIdentifier
    : identifier (Dot identifier)*
    ;

// TOKENS BEGIN HERE

// COMMENT
//     : '//' ~ [\r\n]* -> channel (COMMENT)
//     ;

Int
    : '0'
    | [1-9] [0-9]*
    ;


StringLit
    : 'aaaaaaaaaaaaaaaaaa'
    ;







WS
    : [ \t\r\n\f]+ -> channel (OFF_CHANNEL)
    ;

LEXER_CHAR_SET_BODY
    : (~ [\]\\] | '\\' .)+
    ;
LEXER_CHAR_SET
    : ']'
    ;

Semi: ';';

Lexer: 'lexer';
Grammar: 'Grammar';
Parser: 'Parser';
Rbrace: ']';
Assign: '=';
Dot: '.';
Import: 'Import';
Comma: ',';
At: 'At';
Coloncolon: '::';
Mode: 'Mode';
RuleID: 'Aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';
TokenID: 'Bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb';
Colon: ':';
Catch: 'Catch';
Finally: 'Finally';
Returns: 'Returns';
Throws: 'Throws';
Locals: 'Locals';

// Why Is There Java In My Antlr
Public: 'Public';
Private: 'Private';
Protected: 'Protected';
Fragment: 'Fragment';
OR: '|';
Pound: '#';
Question: '?';
LParen: '(';
RParen: ')';
Rarrow: '->';
LT: '<';
GT: '>';
PlusAssign: '+=';
Star: '*';
Plus: '+';
Not: '~';
Range: '..';