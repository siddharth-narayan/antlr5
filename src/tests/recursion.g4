grammar recursion;

x : A x B // Simple recursion
  | C y
  ;

y : A y B y C
  | D x
  ;

A: 'a';
B: 'b';
C: 'c';
D: 'd';
