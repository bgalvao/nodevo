# Rusty Genetic Programming (rusty-gp)
An implementation of Genetic Programming in Rust. This one pertains to symbolic regression.

## Notes from the author
I just started out programming in Rust and the best way to learn a new programming language is to implement something in it. Since I am mostly acquainted with Genetic Programming and am researching in it, I thought this would be the best way to learn it. That being said, the first implementation is just to get things going! - if this notice is still here, there's still work to be done: with time and knowledge I will definitely optimize the code for speed, readability and proper use of Rust idioms like the Rust gods intended them to be used :).

## OK, but what in the world is Genetic Programming?
It's an evolutionary machine learning algorithm. To get an overall view of how it works, click [here](http://geneticprogramming.com/tutorial/). To just know what it is about, click [here](http://geneticprogramming.com/).

## Can I get this running out of the box?
Not just yet... I need to find a dataset that I can freely use as demo without any issues. Feel free to propose one!

### Some cool resources used to learn Rust
- https://doc.rust-lang.org/book/getting-started.html.
- The awesome [/r/rust](www.reddit.com/r/rust) community!

## TODO
The top priorities are opened in the [Issues](https://github.com/bernardo-galvao/rusty-gp/issues) section. However, given that the author has some goals related to his thesis, it is worth pointing out his plan here.
- use single thread_rng() call to use a single random number generator (eventually use the os-level rng for a speedup)
- implement GSGP
- implement efficient offline evolution of GSGP, according to the work of [Castelli et al. (2014)](http://gsgp.sourceforge.net/)
- Implement Parallel and Distributed GP
- implement MPHGP, thesis work of the author of this repo
- implement extensibility of subpopulation number
- Oracle Genetic Algorithm for Meta-tuning of MPHGP
- Size reduction algorithms
