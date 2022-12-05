Day 05
======

I did not want to spend time writing a parser for the insane data format in
today's puzzle, so I divided it into two files and rewrote the first part by
hand from

```
[H]                 [Z]         [J]
[L]     [W] [B]     [G]         [R]
[R]     [G] [S]     [J] [H]     [Q]
[F]     [N] [T] [J] [P] [R]     [F]
[B]     [C] [M] [R] [Q] [F] [G] [P]
[C] [D] [F] [D] [D] [D] [T] [M] [G]
[J] [C] [J] [J] [C] [L] [Z] [V] [B]
[M] [Z] [H] [P] [N] [W] [P] [L] [C]
 1   2   3   4   5   6   7   8   9 
```

to the much more sensible

```
M,J,C,B,F,R,L,H
Z,C,D
H,J,F,C,N,G,W
P,J,D,M,T,S,B
N,C,D,R,J
W,L,D,Q,P,J,G,Z
P,Z,T,F,R,H
L,V,M,G
C,B,G,P,F,Q,R,J
```

## Issues

I had a lot of trouble simply modifying my data. I stored it in `Vec<Vec<char>>`
and generally pushing to and popping from was a bit of a pain. Need to find out
why I have these issues. I guess it is something with iterators being by ref by
default, and passing a reference from one object's things to another seems to
cause issues.

Second, I was not able to write a function that modifies the vector. I tried

```rust
fn do_something(state: &mut Vec<Vec<char>>) {
    // ...
}

let mut state : Vec<Vec<char>> = Vec::new();
do_something(&mut state);
```

but that did not work.
