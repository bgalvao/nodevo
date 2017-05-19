extern crate rand;
use rand::{thread_rng, Rng};

use core::population::Population;
use core::individual::Individual;
use core::data::Data;
use core::individual::variation::{standard, geometric_semantic};

/// Enum to select the phenotypic search component of the algorithm. It serves to redirect
/// to `pub` functions in the `population` module, where the selection algorithms
/// are defined and other data such as a map of ranks is mantained in order to be
/// able to perform, for example, rank selection.
enum Selection {
    /// `Tournament` selection draws a random sample from the population and picks the fittest.
    Tournament,
    FitnessProportionate, // TODO
    Rank, // TODO
}

/// Enum to select the genotypic search component (variation) of the algorithm.
/// It serves to redirect to the `pub` functions in the `individual` module.
enum Variation {
    /// `Standard` variation methods operate at random points of parent trees.
    Standard,
    /// `GeometricSemantic` methods operate on the semantics
    /// to approximate to targets more quickly,
    /// however they increase the size of offspring dramatically, especially when using crossover.
    /// First field denotes mutation step,
    /// and the second whether this mutation is bounded to the codomain [0,1].
    GeometricSemantic(f32, bool),
}

/// A struct to hold the state of a Genetic Programming (GP) run.
///
/// The actual algorithm running is determined by the fields
/// `selection_method` and `variation_method`. This is a design choice
/// to mantain flexibility of Meta-GA and to conveniently make it
/// possible to get funky setting up a GP
pub struct GP {
    data: Data,
    pop: Population,
    crossover_probability: f32,
    pop_size: usize,
    pool_size: usize,
    selection_method: Selection,
    variation_method: Variation,
}

impl GP {
    /// Returns a new standard `GP`, i.e. `variation_method: Variation::Standard`.
    /// It defaults to crossover rate of 0.9, population size of 100, pool size
    /// of 4, and tournament selection.
    pub fn new_gp(dataset: Data) -> GP {
        GP {
            data: dataset,
            pop: Population::new(),
            crossover_probability: 0.9,
            pop_size: 100,
            pool_size: 4,
            selection_method: Selection::Tournament,
            variation_method: Variation::Standard,
        }
    }

    /// Returns a Geometric Semantic `GP`,
    /// i.e. `variation_method: Variation::GeometricSemantic(1.0, true)`,
    /// with mutation step of 1.0 and bounded by a logistic function
    /// (i.e. random subtrees of GS mutation are fed to a logistic function).
    /// Defaults are the same as of `new_gp(dataset: Data)` except that crossover rate is 0.0.
    pub fn new_gsgp(dataset: Data) -> GP {
        GP {
            data: dataset,
            pop: Population::new(),
            crossover_probability: 0.0,
            pop_size: 100,
            pool_size: 4,
            selection_method: Selection::Tournament,
            variation_method: Variation::GeometricSemantic(1.0, true),
        }
    }

    /// Sets the population to a previously initialized or evolved population `pop`.
    /// Will panic if its size does not match the `pop_size` specified by the GP algorithm,
    /// so be sure to change `pop_size` before performing this operation.
    pub fn set_pop(mut self, pop: Population) -> GP {
        if self.pop_size == pop.size() {
            self.pop = pop;
            self
        } else {
            panic!("@GP::set_pop() Tried to set population to a non-conformant
            with `pop_size` of this GP instance.
            Please correct it before performing this op by using GP::set_pop_size().");
        }
    }

    /// Sets population size.
    pub fn set_pop_size(mut self, size: usize) -> GP {
        self.pop_size = size;
        self
    }

    /// Sets crossover rate, i.e. the rate at which crossover is performed instead of mutation.
    pub fn set_xo_rate(mut self, xo_rate: f32) -> GP {
        self.crossover_probability = xo_rate;
        self
    }

    /// Sets how many individuals are drawn at random for the selection phase of the algorithm
    /// regardless of the `selection_method`
    pub fn set_pool_size(mut self, ps: usize) -> GP {
        self.pool_size = ps;
        self
    }

    /// Picks the selection method according to the options provided by `ènum Selection`
    pub fn set_selection_method(mut self, sm: Selection) -> GP {
        self.selection_method = sm;
        self
    }

    /// Utility to return an immutable reference to the `pop`ulation of `GP`.
    pub fn pop(&self) -> &Population {
        &self.pop
    }

    /// Returns mutable reference to `pop`ulation of `GP`
    pub fn pop_mut(&mut self) -> &mut Population {
        &mut self.pop
    }

    /// Initializes new population using ramped-half-half initialization with maximum depth 6
    /// and to a number of individuals specified by `pop_size`. Will replace existing `pop`ulation!
    pub fn init_new_pop(&mut self) -> () {
        self.pop = Population::new_rhh(self.pop_size, 6, &self.data);
    }

    /// Utility to print to the console the state of evolution of `GP`.
    ///
    /// The state is hereby defined by the Individual with the lowest training
    /// error in the population.
    /// # Usage
    /// Make sure to use this only after calling sort_by_te on `pop`, as this
    /// will retrieve a reference to the first element in `Vec<Individual>`, so it assumes
    /// that it is sorted and thus the fittest individual is the first element.
    fn print_state(&self) {
        let f = self.pop.get_first();
        println!("----------------------");
        println!("train:\t{:?}", f.train().unwrap());
        println!("test:\t{:?}", f.test().unwrap());
        println!("size:\t{:?}", f.size());
        println!("depth:\t{:?}\n\n", f.depth());
    }

    /// Matches `selection_method` to call the corresponding selection function
    /// in `core::population`.
    fn select(&self) -> &Individual {
        match self.selection_method {
            Selection::Tournament => self.pop.tournament_select(self.pool_size),
            Selection::FitnessProportionate => {
                self.pop.fitness_proportionate_select(self.pool_size)
            }
            Selection::Rank => self.pop.rank_select(self.pool_size),
        }
    }

    /// Matches `variation_method` to call the corresponding crossover
    /// function in `core::individual::variation`.
    fn crossover(&self, p1: &Individual, p2: &Individual, data: &Data) -> Individual {
        match self.variation_method {
            Variation::Standard => standard::crossover(p1, p2, data),
            Variation::GeometricSemantic(_, _) => geometric_semantic::crossover(p1, p2, data),
        }
    }

    /// Matches `variation_method` to call the corresponding mutation
    /// function in `core::individual::variation`.
    fn mutation(&self, p1: &Individual, data: &Data) -> Individual {
        match self.variation_method {
            Variation::Standard => standard::mutation(p1, data),
            Variation::GeometricSemantic(step, bounded) => {
                geometric_semantic::mutation(p1, data, step, bounded)
            }
        }
    }

    /// Evolves the `Population` contained in this GP for a `gens` generations.
    pub fn evolve(&mut self, gens: usize) {
        if self.pop.size() == 0 {
            self.init_new_pop();
            println!("self.pop initialized.");
        }
        let mut rng = thread_rng();
        for gen in 0..gens {
            println!("Gen {:?}", gen + 1);
            let mut offspring_pop = Population::new();
            while offspring_pop.size() < self.pop.size() {
                let offspring: Individual;
                let p1 = self.select();
                if rng.next_f32() < self.crossover_probability {
                    let p2 = self.select(); // &Individual
                    offspring = self.crossover(p1, p2, &self.data); // self.crossover!!
                } else {
                    offspring = self.mutation(p1, &self.data);
                }
                offspring_pop.add_individual(offspring);
            }
            self.pop = offspring_pop;
            self.pop.sort_by_te();
            self.print_state();
        } // perhaps print final solution
    }

    /// Removes excess individuals, the less fit.
    ///
    /// Under the hood, calls a function that sorts by te and then truncates a vector.
    pub fn clean(&mut self) {
        self.pop.keep_k_best(self.pop_size);
    }
}
