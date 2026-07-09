startRule
    : compilationUnit EOF
    ;

compilationUnit
    : programUnit+
    ;

programUnit
    : identificationDivision environmentDivision? dataDivision? procedureDivision? programUnit* endProgramStatement?
    ;

endProgramStatement
    : END PROGRAM programName DOT_FS
    ;

// --- identification division --------------------------------------------------------------------

identificationDivision
    : (IDENTIFICATION | ID) DIVISION DOT_FS programIdParagraph identificationDivisionBody*
    ;

identificationDivisionBody
    : authorParagraph
    | installationParagraph
    | dateWrittenParagraph
    | dateCompiledParagraph
    | securityParagraph
    | remarksParagraph
    ;

RPARENCHAR
    : ')'
    ;

SLASHCHAR
    : '/'
    ;

// literals
NONNUMERICLITERAL
    : STRINGLITERAL
    | DBCSLITERAL
    | HEXNUMBER
    | NULLTERMINATED
    ;

fragment HEXNUMBER
    : X '"' [0-9A-F]+ '"'
    | X '\'' [0-9A-F]+ '\''
    ;

fragment NULLTERMINATED
    : Z '"' (~["\n\r] | '""' | '\'')* '"'
    | Z '\'' (~['\n\r] | '\'\'' | '"')* '\''
    ;

fragment STRINGLITERAL
    : '"' (~["\n\r] | '""' | '\'')* '"'
    | '\'' (~['\n\r] | '\'\'' | '"')* '\''
    ;