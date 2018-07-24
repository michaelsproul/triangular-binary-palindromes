Triangular Numbers and Binary Palindromes
====

I was at a 21st birthday party recently and someone remarked that the number 21
is both a triangular number and a binary palindrome, which lead me to wonder if
there is any link between the two concepts. A few examples (0-36) seemed to
bear this theory out, so I took the rather overkill step of writing a program
to check the next couple of hundred thousand... As it happens, 55 is the first
triangular number that can't be written as a binary palindrome.

```
0 = 0
1 = 1
3 = 11
6 = 0110
10 = 01010
15 = 1111
21 = 10101
28 = 0011100
36 = 00100100
45 = 101101
55 = 110111
thread 'main' panicked at 'conjecture is false!', src/main.rs:12:17
```
