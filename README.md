# Rusty Genetic Programming (rusty-gp)
An implementation of Genetic Programming in Rust.

## Personal note from the author
I just started out programming in Rust and the best way to learn a new programming language is to implement something in it. Since I am mostly acquainted with Genetic Programming and am researching in it, I thought this would be the best way to learn it. That being said, the first implementation is just to get things going! - if this notice is still here, there's still work to be done: with time and knowledge acquisition I will definitely optimize the code for speed, readability and proper use of Rust idioms like the Rust gods intended them to be used :). I'm doing my best to become able to think out of the OOP-box.

## OK, but what in the world is Genetic Programming?
It's an evolutionary machine learning algorithm. Look it up.

## Can I get this running out of the box?
Yes, I've included a sample dataset so that there is data to train on.

### Some cool resources used to learn Rust
- https://doc.rust-lang.org/book/getting-started.html.
- The awesome [/r/rust](www.reddit.com/r/rust) community!

## TODO
- use single thread_rng() call to use a single random number generator (eventually use the os-level rng for a speedup)
- organize code in general
- optimize refs to Data, optimize all the loops with closures (aka lambda expressions) + `.map()`s and `.filter()`s
- implement GSGP
- implement efficient offline evolution of GSGP
- implement MPHGP, thesis work of the author of this repo
- implement extensibility of subpopulation number
- Oracle Genetic Algorithm
- Size reduction algorithms
