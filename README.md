# Rusty Genetic Programming (rusty-gp)
An implementation of Genetic Programming in Rust. This one pertains to symbolic regression.

## OK, but what in the world is Genetic Programming?
It's an evolutionary supervised machine learning algorithm. It evolves a population of programs (in this case, a mathematical function) that best maps inputs to outputs.
- to get an overall view of how it works, click [here](http://geneticprogramming.com/tutorial/).

## Can I get this running out of the box?
Yes you can. I included the [yacht dataset](http://archive.ics.uci.edu/ml/datasets/yacht+hydrodynamics) so that there is some data to run on. Credited to the UCI repository:
- Lichman, M. (2013). UCI Machine Learning Repository [http://archive.ics.uci.edu/ml]. Irvine, CA: University of California, School of Information and Computer Science.

## How to get this running?
After [installing](https://www.rust-lang.org/en-US/install.html) Rust on your machine, clone this repo, `cd` into it and run the commands. Something like:
```
git clone https://github.com/bernardo-galvao/rusty-gp.git
cd rusty-gp
cargo build --release
cargo run
```

### Some cool resources used to learn Rust
- https://doc.rust-lang.org/book/getting-started.html.
- The awesome [/r/rust](www.reddit.com/r/rust) community!

## Notes from the author
I just started out programming in Rust and the best way to learn a new programming language is to implement something in it. Since I am mostly acquainted with Genetic Programming and am researching in it, I thought this would be the best way to learn it. That being said, this is a work in progress. - with time and knowledge optimize the code will be optimized.

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
