The grammar for rules of the form $S := X S Y | Z$ is not regular, and can't be represented as a Regex/dfa

Let's take this grammar:

$
  &T: S X^* \
  &S: Z S X | 'b a' | 'a b' \
  &Z: 'a' | 'b' \
  &X: 'n'
$

on input `'abbaaabbabn'`

Even after running prediction on S we might need to fall back to T to see if S continues correcetly.
With input `'aban'`, depending on the order of  Alts, we could match 'ab' as S, but we would then go back and see X\* (and EOF) are not matched

Multiple self recursive alts can be rewritten as 