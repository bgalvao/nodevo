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
# only clone last commit!
git clone --depth 1 https://github.com/bravo-9/rusty-gp.git
cd rusty-gp
cargo build --release
cargo run
```

## How to play around?
This describes the currently available functionality, so it's just a preview. In the `main.rs` go to `main()`:
```rust
let mut gsgp = GP::new_gsgp(ds)
                    .set_pop_size(150)
                    .set_pool_size(5)
                    .set_xo_rate(0.0);
gsgp.init_new_pop();
gsgp.evolve(100);
```
Note that `new_gp()` and `new_gsgp()` will initialize a standard GP and a Geometric Semantic GP respectively, according to the following defaults:
- `pop_size: 100` (population_size)
- `pool_size: 3`: how many individuals one is drawing at random from the population for selection for the variation phase.
- `xo_rate: 0.9`: rate of crossover. Rate of mutation is implicitly `1 - xo_rate`, and only one type of variation takes place. If you're doing Geometric Semantic GP (i.e. `new_gsgp()`) keep this as low as `0.0`!
- further description will come soon as the implementation gets more solid.

## Notes from the author
I just started out programming in [Rust](www.rust-lang.org) and the best way to learn a new programming language is to implement something in it. Since I am mostly acquainted with Genetic Programming and am researching in it, I thought this would be the best way to learn it. That being said, this is a work in progress. - with time and knowledge optimize the code will be optimized. Thanks to the [community](www.reddit.com/r/rust).

## TODO
The top priorities are opened in the [Issues](https://github.com/bernardo-galvao/rusty-gp/issues) section. However, given that the author has some goals related to his thesis, it is worth pointing out his plan here.
- use single thread_rng() call to use a single random number generator (eventually use the os-level rng for a speedup)
- ~~implement GSGP~~ :muscle: **reconstruction ability do be implemented** according to the work of [Castelli et al. (2014)](http://gsgp.sourceforge.net/)
- Implement Parallel and Distributed GP
- implement MPHGP, thesis work of the author of this repo
- implement extensibility of subpopulation number
- Oracle Genetic Algorithm for Meta-tuning of MPHGP
- Size reduction algorithms
