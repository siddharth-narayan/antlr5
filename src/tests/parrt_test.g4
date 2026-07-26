// Stolen with love from https://theantlrguy.atlassian.net/wiki/spaces/~admin/pages/524294/LL+grammar+analysis

grammar parrt_test;

x : A* B X
  | A* C Y
  ;

s : e X
  | e Y
  ;
 
e : L e R
  | I
  ; 

A: 'a';
B: 'b';
C: 'c';
X: 'x';
Y: 'y';
L: 'l';
R: 'r';
I: 'i';