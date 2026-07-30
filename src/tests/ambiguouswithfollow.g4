grammar ambiguouswithfollow;

x: ambig B C D ;
ambig: A | A B N ;

test: A | A ;

A: 'a';
B: 'b';
C: 'c';
D: 'd';
N: 'n';