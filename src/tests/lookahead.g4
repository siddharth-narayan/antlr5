grammar lookahead;

t: s x*;

s: z s x | LETTERB 'x' LETTERA 'y' | LETTERA 'y' LETTERB 'x';

z: LETTERA 'x' | LETTERB 'y';

x: LETTERN 'z';

LETTERA: 'a';
LETTERB: 'b';
LETTERN: 'n';