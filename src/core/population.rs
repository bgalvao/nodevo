use rand::{thread_rng, Rng};

use core::individual::Individual;
use core::data::Data;

/// A container for a group of individuals.
#[derive(Debug)]
pub struct Population {
    core: Vec<Individual>,
}

impl Population {
    /// Returns size of the population.
    pub fn size(&self) -> usize {
        self.core.len()
    } // does not need to check for mutation if it ain't mutating!

    /// Returns empty Population
    pub fn new() -> Population {
        Population { core: vec![] }
    }

    /// Returns `true` in case there are 0 Individuals in the population.
    pub fn is_empty(&self) -> bool {
        self.core.len() == 0
    }

    /// Adds an owned `Individual` to the population.
    pub fn add_individual(&mut self, new_guy: Individual) {
        self.core.push(new_guy);
    }

    /// Adds individuals in a batch.
    ///
    /// By the specification of `new_guys`, the `Individual`s must be owned.
    pub fn add_individuals(&mut self, new_guys: Vec<Individual>) {
        // doing self.core.append(new_guys) failed because appends takes in a mutable reference...
        for i in new_guys.into_iter() {
            self.core.push(i);
        }
    }

    /// Returns a mutable reference to the `core` of `Population`.
    pub fn core_mut(&mut self) -> &mut Vec<Individual> {
        &mut self.core
    }

    /// Returns a reference to the `core` of `Population`.
    pub fn core(&self) -> &Vec<Individual> {
        &self.core
    }

    /// Initializes a filled Population using ramped-half-half initialization
    pub fn new_rhh(pop_size: usize, max_init_depth: usize, data: &Data) -> Population {
        let mut p = Population::new();
        // Note: depth at root node is 0.
        // hence, #depths = #depth_groups = max_init_depth (maximum initial depth)
        let indivs_per_depth = pop_size / max_init_depth;
        let remaining_indivs = pop_size % max_init_depth; // remainder, not modulus

        let mut grow_indivs = ((indivs_per_depth as f32) / 2.0).floor() as i32;
        let mut full_indivs = ((indivs_per_depth as f32) / 2.0).ceil() as i32;

        for depth in 1..(max_init_depth + 1) {
            // if at the last depth group, overwrite to include the poor remaining_indivs
            if depth == max_init_depth {
                grow_indivs = (((indivs_per_depth as f32) + (remaining_indivs as f32)) / 2.0)
                    .floor() as i32;
                full_indivs = (((indivs_per_depth as f32) + (remaining_indivs as f32)) / 2.0)
                    .ceil() as i32;
            }
            // fill depth group
            for _ in 0..full_indivs {
                let mut i = Individual::full(depth, data);
                i.compute_semantics(data); // only necessary for offline GSGP
                i.compute_depth(); // only necessary for offline GSGP
                p.core.push(i);
            }

            for _ in 0..grow_indivs {
                let mut i = Individual::grow(depth, data);
                i.compute_semantics(data); // only necessary for offline GSGP
                i.compute_depth(); // only necessary for offline GSGP
                p.core.push(i);
            }
        }
        p
    }

    /// Sorts population by training error.
    pub fn sort_by_te(&mut self) {
        // holy mother of hack
        self.core
            .sort_by_key(|i| {
                             (i.train()
                                  .expect("@Population::sort_by_te()
        an Individual does not have training error computed.") *
                              1_000_000f32) as u64
                         });
    }

    pub fn get_first(&self) -> &Individual {
        &self.core[0]
    }

    pub fn clone_k_best(&self, k: usize) -> Vec<Individual> {
        self.core.iter().take(k).map(|i| i.clone()).collect()
    }

    pub fn keep_k_best(&mut self, k: usize) {
        self.sort_by_te();
        self.core.truncate(k);
    }

    //  ----------------------------------------------------------------------  Selection methods
    /// Performs tournament selection in this `Population`.
    ///
    /// - Draws a random sample.
    /// - Takes the fittest from the sample and returns a reference to it.
    pub fn tournament_select(&self, pool_size: usize) -> &Individual {
        let mut rng = thread_rng();
        let mut first_guy = &self.core[rng.gen_range(0, self.size())];
        for _ in 0..(pool_size - 1) {
            // There's possibly a closure for this with iter magic? :3
            let new_guy = &self.core[rng.gen_range(0, self.size())];
            if new_guy.train() < first_guy.train() {
                first_guy = new_guy;
            }
        }
        first_guy
    }

    /// Performs fitness proportional selection in this `Population`.
    ///
    /// - Draws a random sample.
    /// - Calculates fitness share for each individual.
    /// - Probabilistically selects an individual and returns a reference to it.
    pub fn fitness_proportionate_select(&self, pool_size: usize) -> &Individual {
        unimplemented!();
    }

    /// Performs rank selection in this `Population`
    ///
    /// - Draws a random sample.
    /// - Determines ranks within the sample.
    /// - Calculates rank-share for each individual
    /// - Probabilistically selects an individual and returns a reference to it
    pub fn rank_select(&self, pool_size: usize) -> &Individual {
        unimplemented!();
    }

    pub fn pareto_rank_select(&self, pool_size: usize) -> &Individual {
        unimplemented!();
    }
}
