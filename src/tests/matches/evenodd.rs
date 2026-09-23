static x: &'static str = 
    "
    grammar x;
    
    first:  (A A)* B | A* B ;
    A: 'a' ;
    B: 'b' ;
    ";
