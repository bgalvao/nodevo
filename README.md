# Nodevo
An implementation of Genetic Programming in Rust. This one pertains to symbolic regression. This is supposed to become a flexible library and a better implementation of the work of my thesis (which slides are found [here](https://docs.google.com/presentation/d/1-T-MNxm-bCozN1VtoF7VLlWRhL_eEkn-U5fpIf8s10U/edit?usp=sharing))

## OK, but what in the world is Genetic Programming?
It's an evolutionary supervised machine learning algorithm. It evolves a population of programs (in this case, a mathematical function) that best maps inputs to outputs.
- to get an overall view of how it works, click [here](http://geneticprogramming.com/tutorial/).

## Can I get this running out of the box?
Yes you can. I included the [yacht dataset](http://archive.ics.uci.edu/ml/datasets/yacht+hydrodynamics) so that there is some data to run on. Credited to the UCI repository:
- Lichman, M. (2013). UCI Machine Learning Repository [http://archive.ics.uci.edu/ml]. Irvine, CA: University of California, School of Information and Computer Science.

## How to get this running?
After [installing](https://www.rust-lang.org/en-US/install.html) Rust on your machine, clone this repo, `cd` into it and run the commands. Something like:
```bash
git clone https://github.com/bgalvao/nodevo.git
cd nodevo
cargo build --release
cargo run
```

## How to play around?
This describes the currently available functionality, so it's just a preview. In the `main.rs` go to `main()`:

### __Genetic Programming (GP)__ (standard)
```rust
let ds = Data::new("yacht");
let mut gp = GP::new_gp(ds)
                    .set_pop_size(150)
                    .set_pool_size(5)
                    .set_xo_rate(0.8);
gp.init_new_pop();
gp.evolve(100);
```

### __Geometric Semantic Genetic Programming (GSGP)__
```rust
let ds = Data::new("yacht");
let mut gsgp = GP::new_gsgp(ds)
                    .set_pop_size(150)
                    .set_pool_size(5)
                    .set_xo_rate(0.0);
gsgp.init_new_pop();
gsgp.evolve(100);
```

Note that `new_gp()` and `new_gsgp()` will initialize, according to the following defaults:
- `pop_size: 100` (population_size)
- `pool_size: 3`: how many individuals one is drawing at random from the population for selection for the variation phase.
- `xo_rate: 0.9`: rate of crossover. Rate of mutation is implicitly `1 - xo_rate`, and only one type of variation takes place. If you're doing Geometric Semantic GP (i.e. `new_gsgp()`) you're recommended to keep this as low as `0.0`!

### __Parallel and Distributed Genetic Programming__
This is a Genetic Programming system that distributes computation over subpopulations. For now only with standard Genetic Programming, and with time, hybrid systems shall be supported as soon as reconstruction of GSGP `Individual`s is implemented. First declare the `GP` subpopulations that you want to be included:
```rust
let ds = Data::new("yacht");
let gp1 = GP::new_gp(ds.clone())
                    .set_pop_size(50)
                    .set_pool_size(5)
                    .set_xo_rate(0.8);


let gp2 = GP::new_gp(ds)
                    .set_pop_size(100)
                    .set_pool_size(5)
                    .set_xo_rate(0.8);
```
Finally declare a new `Mgp` (multi-gp) that takes in the declared subpopulations:
```rust
let mut mgp = Mgp::new().add_subpop(gp1)
                        .add_subpop(gp2);
mgp.init();
mgp.evolve_in_parallel(3, 30);
// evolves two `GP` subpopulations in parallel for 3 turns of 30 generations.
```

## Notes from the author
I just started out programming in [Rust](www.rust-lang.org) and the best way to learn a new programming language is to implement something in it. Since I am mostly acquainted with Genetic Programming and am researching in it, I thought this would be the best way to learn it. That being said, this is a work in progress - with time and knowledge the code will be optimized. Thanks to the [community](www.reddit.com/r/rust).

## TODO
The top priorities are opened in the [Issues](https://github.com/bernardo-galvao/rusty-gp/issues) section. However, given that the author has some goals related to his thesis, it is worth pointing out his plan and expectable features here.

:white_medium_square: Implement GSGP :ok::muscle:; **reconstruction ability do be implemented** according to the work of [Castelli et al. (2014)](http://gsgp.sourceforge.net/)

:white_medium_square: Implement Parallel and Distributed GP :ok:; **number of migrants to be specified by user**

:white_medium_square: Oracle Genetic Algorithm for Meta-tuning of MPHGP

:white_medium_square: Size reduction algorithms
