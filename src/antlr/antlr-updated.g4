grammar antlr;

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
    ;

// This is the list of all constructs that can be declared before
// the set of rules that compose the grammar, and is invoked 0..n
// times by the grammarPrequel rule.

prequelConstruct
    : optionsSpec
    | delegateGrammars
    | tokensSpec
    | channelsSpec
    | action_
    ;

// ------------
// Options - things that affect analysis and/or code generation

optionsSpec
    : OPTIONS (option SEMI)* RBRACE
    ;

option
    : identifier ASSIGN optionValue
    ;

optionValue
    : identifier (DOT identifier)*
    | STRING_LITERAL
    | actionBlock
    | INT
    ;

// ------------
// Delegates

delegateGrammars
    : IMPORT delegateGrammar (COMMA delegateGrammar)* SEMI
    ;

delegateGrammar
    : identifier ASSIGN identifier
    | identifier
    ;

// ------------
// Tokens & Channels

tokensSpec
    : TOKENS idList? RBRACE
    ;

channelsSpec
    : CHANNELS idList? RBRACE
    ;

idList
    : identifier (COMMA identifier)* COMMA?
    ;

// Match stuff like @parser::members {int i;}

action_
    : AT (actionScopeName COLONCOLON)? identifier actionBlock
    ;

// Scope names could collide with keywords; allow them as ids for action scopes

actionScopeName
    : identifier
    | LEXER
    | PARSER
    ;

actionBlock
    : ACTION
    ;

argActionBlock
    : BEGIN_ARGUMENT ARGUMENT_CONTENT*? END_ARGUMENT
    ;

modeSpec
    : MODE identifier SEMI lexerRuleSpec*
    ;

rules
    : ruleSpec*
    ;

ruleSpec
    : parserRuleSpec
    | lexerRuleSpec
    ;

parserRuleSpec
    : ruleModifiers? RULEID argActionBlock? ruleReturns? throwsSpec? localsSpec? rulePrequel* COLON ruleBlock SEMI
        exceptionGroup
    ;

exceptionGroup
    : exceptionHandler* finallyClause?
    ;

exceptionHandler
    : CATCH argActionBlock actionBlock
    ;

finallyClause
    : FINALLY actionBlock
    ;

rulePrequel
    : optionsSpec
    | ruleAction
    ;

ruleReturns
    : RETURNS argActionBlock
    ;

// --------------
// Exception spec
throwsSpec
    : THROWS qualifiedIdentifier (COMMA qualifiedIdentifier)*
    ;

localsSpec
    : LOCALS argActionBlock
    ;

/** Match stuff like @init {int i;} */
ruleAction
    : AT identifier actionBlock
    ;

ruleModifiers
    : ruleModifier+
    ;

// An individual access modifier for a rule. The 'fragment' modifier
// is an internal indication for lexer rules that they do not match
// from the input but are like subroutines for other lexer rules to
// reuse for certain lexical patterns. The other modifiers are passed
// to the code generation templates and may be ignored by the template
// if they are of no use in that language.

ruleModifier
    : PUBLIC
    | PRIVATE
    | PROTECTED
    | FRAGMENT
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
    : FRAGMENT? TOKENID optionsSpec? COLON lexerRuleBlock SEMI
    ;

lexerRuleBlock
    : lexerAltList
    ;

lexerAltList
    : lexerAlt (OR lexerAlt)*
    ;

lexerAlt
    : lexerElements lexerCommands?
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
    | actionBlock QUESTION?
    ;

// but preds can be anywhere

lexerBlock
    : LPAREN lexerAltList RPAREN
    ;

// E.g., channel(HIDDEN), skip, more, mode(INSIDE), push(INSIDE), pop

lexerCommands
    : RARROW lexerCommand (COMMA lexerCommand)*
    ;

lexerCommand
    : lexerCommandName LPAREN lexerCommandExpr RPAREN
    | lexerCommandName
    ;

lexerCommandName
    : identifier
    | MODE
    ;

lexerCommandExpr
    : identifier
    | INT
    ;

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
    | actionBlock QUESTION? predicateOptions?
    ;

predicateOptions
    : LT predicateOption (COMMA predicateOption)* GT
    ;

predicateOption
    : elementOption
    | identifier ASSIGN (actionBlock | INT | STRING_LITERAL)
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
    | STRING_LITERAL elementOptions?
    | characterRange
    | LEXER_CHAR_SET
    ;

// -------------
// Grammar Block
block
    : LPAREN (optionsSpec? ruleAction* COLON)? altList RPAREN
    ;

// ----------------
// Parser rule ref
ruleref
    : RULEID argActionBlock? elementOptions?
    ;

// ---------------
// Character Range
characterRange
    : STRING_LITERAL RANGE STRING_LITERAL
    ;

terminalDef
    : TOKENID elementOptions?
    | STRING_LITERAL elementOptions?
    ;

// Terminals may be adorned with certain options when
// reference in the grammar: TOK<,,,>
elementOptions
    : LT elementOption (COMMA elementOption)* GT
    ;

elementOption
    : qualifiedIdentifier
    | identifier ASSIGN (qualifiedIdentifier | STRING_LITERAL | INT)
    ;

identifier
    : RULEID
    | TOKENID
    ;

qualifiedIdentifier
    : identifier (DOT identifier)*
    ;

// TOKENS BEGIN HERE

DOC_COMMENT
    : '/**' .*? ('*/' | EOF) -> channel (COMMENT)
    ;

BLOCK_COMMENT
    : '/*' .*? ('*/' | EOF) -> channel (COMMENT)
    ;

LINE_COMMENT
    : '//' ~ [\r\n]* -> channel (COMMENT)
    ;

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
STRING_LITERAL
    : '\'' (ESC_SEQUENCE | ~ ['\r\n\\])* '\''
    ;

UNTERMINATED_STRING_LITERAL
    : '\'' (ESC_SEQUENCE | ~ ['\r\n\\])*
    ;

// -------------------------
// Arguments
//
// Certain argument lists, such as those specifying call parameters
// to a rule invocation, or input parameters to a rule specification
// are contained within square brackets.
BEGIN_ARGUMENT
    : '['
    ;

// Many language targets use {} as block delimiters and so we
// must recursively match {} delimited blocks to balance the
// braces. Additionally, we must make some assumptions about
// literal string representation in the target language. We assume
// that they are delimited by ' or " and so consume these
// in their own alts so as not to inadvertently match {}.
ACTION
    : NESTED_ACTION
    ;

fragment NESTED_ACTION
    : // Action and other blocks start with opening {
    '{' (
        NESTED_ACTION          // embedded {} block
        | STRING_LITERAL       // single quoted string
        | DoubleQuoteLiteral   // double quoted string
        | TripleQuoteLiteral   // string literal with triple quotes
        | BacktickQuoteLiteral // backtick quoted string
        | '/*' .*? '*/'        // block comment
        | '//' ~[\r\n]*        // line comment
        | '\\' .               // Escape sequence
        | ~[\\"'`{]
    )*? '}'
    ;

// -------------------------
// Keywords
//
// 'options', 'tokens', and 'channels' are considered keywords
// but only when followed by '{', and considered as a single token.
// Otherwise, the symbols are tokenized as RULEID and allowed as
// an identifier in a labeledElement.
OPTIONS
    : 'options' WS* '{'
    ;

TOKENS
    : 'tokens' WS* '{'
    ;

CHANNELS
    : 'channels' WS* '{'
    ;

// -------------------------
// Punctuation





// -------------------------
// Identifiers - allows unicode rule/token names

ID
    : NameStartChar NameChar*
    ;

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

// E.g., [int x, List<String> a[]]
NESTED_ARGUMENT
    : '['
    ;

ARGUMENT_ESCAPE
    : '\\' .
    ;

ARGUMENT_STRING_LITERAL
    : DoubleQuoteLiteral
    ;

ARGUMENT_CHAR_LITERAL
    : STRING_LITERAL
    ;

END_ARGUMENT
    : ']'
    ;

// added this to return non-EOF token type here. EOF does something weird
UNTERMINATED_ARGUMENT
    : EOF
    ;

ARGUMENT_CONTENT
    : .
    ;

// -------------------------
// mode LexerCharSet;

LEXER_CHAR_SET_BODY
    : (~ [\]\\] | '\\' .)+
    ;

LEXER_CHAR_SET
    : ']'
    ;

UNTERMINATED_CHAR_SET
    : EOF
    ;

// ------------------------------------------------------------------------------
// Grammar specific Keywords, Punctuation, etc.

fragment ESC_SEQUENCE
    : '\\' ([btnfr"'\\] | UnicodeESC | . | EOF)
    ;

fragment HexDigit
    : [0-9a-fA-F]
    ;

fragment UnicodeESC
    : 'u' (HexDigit (HexDigit (HexDigit HexDigit?)?)?)?
    ;

fragment DoubleQuoteLiteral
    : '"' (ESC_SEQUENCE | ~["\r\n\\])*? '"'
    ;

fragment TripleQuoteLiteral
    : '"""' (ESC_SEQUENCE | .)*? '"""'
    ;

fragment BacktickQuoteLiteral
    : '`' (ESC_SEQUENCE | ~["\r\n\\])*? '`'
    ;

// -----------------------------------
// Character ranges

fragment NameChar
    : NameStartChar
    | [0-9]
    | '_'
    | '\u00B7'
    | [\u0300-\u036F]
    | [\u203F-\u2040]
    ;

fragment NameStartChar
    : [A-Z]
    | [a-z]
    | [\u00C0-\u00D6]
    | [\u00D8-\u00F6]
    | [\u00F8-\u02FF]
    | [\u0370-\u037D]
    | [\u037F-\u1FFF]
    | [\u200C-\u200D]
    | [\u2070-\u218F]
    | [\u2C00-\u2FEF]
    | [\u3001-\uD7FF]
    | [\uF900-\uFDCF]
    | [\uFDF0-\uFFFD]
    // ignores | ['\u10000-'\uEFFFF]
    ;

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
TOKENID: 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';
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