grammar recursion;

x : A x B // Simple recursion
  | C y
  ;

y : A x B x C
  | D x
  ;

n: 'a' ;

z: n ;

A: 'a';
B: 'b';
C: 'c';
D: 'd';
