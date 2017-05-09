extern crate rand;
use rand::{thread_rng, Rng};

use core::population::Population;
use core::individual::Individual;
use core::data::Data;
use core::individual::variation::{standard, geometric_semantic};

enum Variation {
    Standard,
    GeometricSemantic(f32, bool), // bounded_mutation and mutation step in reverse
}

/// Phenotypic search component of the algorithm.
enum Selection {
    Tournament,
    //FitnessProportionate, // TODO
    //Rank, // TODO
}

/// Standard Genetic Programming.
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

    pub fn new_gsgp(dataset: Data) -> GP {
        GP {
            data: dataset,
            pop: Population::new(),
            crossover_probability: 0.9,
            pop_size: 100,
            pool_size: 4,
            selection_method: Selection::Tournament,
            variation_method: Variation::GeometricSemantic(1.0, true),
        }
    }
    // BUILDER FUNCTIONS
    //pub fn set_pop(mut self, pop: Population) -> GP {self.pop = pop; self}
    pub fn set_pop_size(mut self, size: usize) -> GP {self.pop_size = size; self}
    pub fn set_xo_rate(mut self, xo_rate: f32) -> GP {self.crossover_probability = xo_rate; self}
    pub fn set_pool_size(mut self, ps: usize) -> GP {self.pool_size = ps; self}
    pub fn set_selection_method(mut self, sm: Selection) -> GP {self.selection_method = sm; self}
    //pub fn set_variation_method(&mut self, vm: Variation) {self.variation_method = vm;}

    pub fn pop(&self) -> &Population {&self.pop}

    // Initialization
    pub fn init_new_pop(&mut self) -> () {self.pop = Population::new_rhh(self.pop_size, 6, &self.data);}

    // State
    fn print_state(&self) {
        let f = self.pop.fittest_extensive_search();
        println!("----------------------");
        println!("train:\t{:?}", f.train().unwrap());
        println!("test:\t{:?}", f.test().unwrap());
        println!("size:\t{:?}", f.size());
        println!("depth:\t{:?}\n\n", f.depth());
    }

    // Evolution
    fn select(&self) -> &Individual {
        match self.selection_method {
            Selection::Tournament => self.pop.tournament_select(self.pool_size),
            //_ => panic!("SORRY: This selection method not yet implemented. Please opt for Selection::Tournament."),
        }
    }

    fn crossover(&self, p1: &Individual, p2: &Individual, data: &Data) -> Individual {
        match self.variation_method {
            Variation::Standard => standard::crossover(p1, p2, data),
            Variation::GeometricSemantic(_, _) => geometric_semantic::crossover(p1, p2, data),
        }
    }

    fn mutation(&self, p1: &Individual, data: &Data) -> Individual {
        match self.variation_method {
            Variation::Standard => standard::mutation(p1, data),
            Variation::GeometricSemantic(step, bounded) => geometric_semantic::mutation(p1, data, step, bounded),
        }
    }

    pub fn evolve(&mut self, gens: usize) {
        let mut rng = thread_rng();
        for gen in 0..gens {
            println!("Gen {:?}", gen+1);
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
            self.print_state();
        } // perhaps print final solution
    }

}
