grammar antlr;

// The main entry point for parsing a v4 grammar.
grammarSpec
    : grammarDecl rules EOF
    ;

grammarDecl
    : grammarType identifier SEMI
    ;

grammarType
    : LEXER GRAMMAR
    | PARSER GRAMMAR
    | GRAMMAR
    ;

rules
    : ruleSpec*
    ;

ruleSpec
    : parserRuleSpec
    | lexerRuleSpec
    ;

parserRuleSpec
    : RULEID COLON ruleBlock SEMI
    ;

ruleBlock
    : ruleAltList
    ;

ruleAltList
    : labeledAlt (OR labeledAlt)*
    ;

labeledAlt
    : alternative (POUND identifier)?
    ;

// --------------------
// Lexer rules

lexerRuleSpec
    : FRAGMENT? TOKENID COLON lexerRuleBlock SEMI
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
    : LPAREN lexerAltList RPAREN
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
    : identifier (ASSIGN | PLUS_ASSIGN) (atom | block)
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
    : QUESTION QUESTION?
    | STAR QUESTION?
    | PLUS QUESTION?
    ;

lexerAtom
    : characterRange
    | terminalDef
    | notSet
    | LEXER_CHAR_SET
    | wildcard
    ;

atom
    : terminalDef
    | ruleref
    | notSet
    | wildcard
    ;

wildcard
    : DOT elementOptions?
    ;

// --------------------
// Inverted element set
notSet
    : NOT setElement
    | NOT blockSet
    ;

blockSet
    : LPAREN setElement (OR setElement)* RPAREN
    ;

setElement
    : TOKENID elementOptions?
    | StringLit elementOptions?
    | characterRange
    ;

// -------------
// Grammar Block
block
    : LPAREN altList RPAREN
    ;

// ----------------
// Parser rule ref
ruleref
    : RULEID elementOptions?
    ;

// ---------------
// Character Range
characterRange
    : StringLit RANGE StringLit
    ;

terminalDef
    : TOKENID elementOptions?
    | StringLit elementOptions?
    ;

// Terminals may be adorned with certain options when
// reference in the grammar: TOK<,,,>
elementOptions
    : LT elementOption (COMMA elementOption)* GT
    ;

elementOption
    : qualifiedIdentifier
    | identifier ASSIGN (qualifiedIdentifier | StringLit | INT)
    ;

identifier
    : RULEID
    | TOKENID
    ;

qualifiedIdentifier
    : identifier (DOT identifier)*
    ;

// TOKENS BEGIN HERE

// DOC_COMMENT
//     : '/**' .*? ('*/' | EOF) -> channel (COMMENT)
//     ;

// BLOCK_COMMENT
//     : '/*' .*? ('*/' | EOF) -> channel (COMMENT)
//     ;

// COMMENT
//     : '//' ~ [\r\n]* -> channel (COMMENT)
//     ;

// -------------------------
// Integer

INT
    : '0'
    | [1-9] [0-9]*
    ;

// -------------------------
// Literal string
//
// ANTLR makes no distinction between a single character literal and a
// multi-character string. All literals are single quote delimited and
// may contain unicode escape sequences of the form \uxxxx, where x
// is a valid hexadecimal number (per Unicode standard).
StringLit
    : 'aaaaaaaaaaaaaaaaaa'
    ;

// UNTERMINATED_StringLit
//     : '\'' (ESC_SEQUENCE | ~ ['\r\n\\])*
//     ;

// // -------------------------
// // Arguments
// //
// // Certain argument lists, such as those specifying call parameters
// // to a rule invocation, or input parameters to a rule specification
// // are contained within square brackets.
// BEGIN_ARGUMENT
//     : '['
//     ;

// // Many language targets use {} as block delimiters and so we
// // must recursively match {} delimited blocks to balance the
// // braces. Additionally, we must make some assumptions about
// // literal string representation in the target language. We assume
// // that they are delimited by ' or " and so consume these
// // in their own alts so as not to inadvertently match {}.
// ACTION
//     : NESTED_ACTION
//     ;

// fragment NESTED_ACTION
//     : // Action and other blocks start with opening {
//     '{' (
//         NESTED_ACTION          // embedded {} block
//         | StringLit       // single quoted string
//         | DoubleQuoteLiteral   // double quoted string
//         | TripleQuoteLiteral   // string literal with triple quotes
//         | BacktickQuoteLiteral // backtick quoted string
//         | '/*' .*? '*/'        // block comment
//         | '//' ~[\r\n]*        // line comment
//         | '\\' .               // Escape sequence
//         | ~[\\"'`{]
//     )*? '}'
//     ;

// -------------------------
// Keywords
//
// 'options', 'tokens', and 'channels' are considered keywords
// but only when followed by '{', and considered as a single token.
// Otherwise, the symbols are tokenized as RULEID and allowed as
// an identifier in a labeledElement.
// OPTIONS
//     : 'options' WS* '{'
//     ;

// TOKENS
//     : 'tokens' WS* '{'
//     ;

// CHANNELS
//     : 'channels' WS* '{'
//     ;

// -------------------------
// Punctuation





// -------------------------
// Identifiers - allows unicode rule/token names

// ID
//     : NameStartChar NameChar*
//     ;

// -------------------------
// Whitespace

WS
    : [ \t\r\n\f]+ -> channel (OFF_CHANNEL)
    ;

// ======================================================
// Lexer modes
// -------------------------
// Arguments
// mode Argument;

// // E.g., [int x, List<String> a[]]
// NESTED_ARGUMENT
//     : '['
//     ;

// ARGUMENT_ESCAPE
//     : '\\' .
//     ;

// ARGUMENT_StringLit
//     : DoubleQuoteLiteral
//     ;

// ARGUMENT_CHAR_LITERAL
//     : StringLit
//     ;

// END_ARGUMENT
//     : ']'
//     ;

// // added this to return non-EOF token type here. EOF does something weird
// UNTERMINATED_ARGUMENT
//     : EOF
//     ;

// ARGUMENT_CONTENT
//     : .
//     ;

// -------------------------
// mode LexerCharSet;

LEXER_CHAR_SET_BODY
    : (~ [\]\\] | '\\' .)+
    ;
LEXER_CHAR_SET
    : ']'
    ;

// UNTERMINATED_CHAR_SET
//     : EOF
//     ;

// ------------------------------------------------------------------------------
// Grammar specific Keywords, Punctuation, etc.

// fragment ESC_SEQUENCE
//     : '\\' ([btnfr"'\\] | UnicodeESC | . | EOF)
//     ;

// fragment HexDigit
//     : [0-9a-fA-F]
//     ;

// fragment UnicodeESC
//     : 'u' (HexDigit (HexDigit (HexDigit HexDigit?)?)?)?
//     ;

// fragment DoubleQuoteLiteral
//     : '"' (ESC_SEQUENCE | ~["\r\n\\])*? '"'
//     ;

// fragment TripleQuoteLiteral
//     : '"""' (ESC_SEQUENCE | .)*? '"""'
//     ;

// fragment BacktickQuoteLiteral
//     : '`' (ESC_SEQUENCE | ~["\r\n\\])*? '`'
//     ;

// // -----------------------------------
// // Character ranges

// fragment NameChar
//     : NameStartChar
//     | [0-9]
//     | '_'
//     | '\u00B7'
//     | [\u0300-\u036F]
//     | [\u203F-\u2040]
//     ;

// fragment NameStartChar
//     : [A-Z]
//     | [a-z]
//     | [\u00C0-\u00D6]
//     | [\u00D8-\u00F6]
//     | [\u00F8-\u02FF]
//     | [\u0370-\u037D]
//     | [\u037F-\u1FFF]
//     | [\u200C-\u200D]
//     | [\u2070-\u218F]
//     | [\u2C00-\u2FEF]
//     | [\u3001-\uD7FF]
//     | [\uF900-\uFDCF]
//     | [\uFDF0-\uFFFD]
//     // ignores | ['\u10000-'\uEFFFF]
//     ;

SEMI: ';';

LEXER: 'lexer';
GRAMMAR: 'grammar';
PARSER: 'parser';
RBRACE: ']';
ASSIGN: '=';
DOT: '.';
IMPORT: 'import';
COMMA: ',';
AT: 'at';
COLONCOLON: '::';
MODE: 'mode';
RULEID: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';
TOKENID: 'bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb';
COLON: ':';
CATCH: 'catch';
FINALLY: 'finally';
RETURNS: 'returns';
THROWS: 'throws';
LOCALS: 'locals';

// WHY IS THERE JAVA IN MY ANTLR
PUBLIC: 'public';
PRIVATE: 'private';
PROTECTED: 'protected';
FRAGMENT: 'fragment';
OR: '|';
POUND: '#';
QUESTION: '?';
LPAREN: '(';
RPAREN: ')';
RARROW: '->';
LT: '<';
GT: '>';
PLUS_ASSIGN: '+=';
STAR: '*';
PLUS: '+';
NOT: '~';
RANGE: '..';