use core::individual::Individual;
use core::data::Data;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Population {
    core: Vec<Individual>,
}

impl Population {

    /// Returns size of the population.
    pub fn size(&self) -> usize {self.core.len()} // does not need to check for mutation if it ain't mutating!

    /// Returns empty Population
    pub fn new() -> Population {Population{core: vec![]}}

    /// Returns `true` in case there are 0 Individuals in the population.
    pub fn is_empty(&self) -> bool {self.core.len() == 0}

    /// Adds an individual to the population
    pub fn add_individual(&mut self, new_guy: Individual) {
        self.core.push(new_guy);
    }
    // let's see about mutating an immutable self.

    /// Initializes a filled Population using ramped-half-half initialization
    pub fn new_rhh(pop_size: usize, max_init_depth: usize, data: &Data) -> Population {
        let mut p = Population::new();
        // Note: depth at root node is 0.
        // hence, #depths = #depth_groups = max_init_depth (maximum initial depth)
        let indivs_per_depth = pop_size / max_init_depth;
        let remaining_indivs = pop_size % max_init_depth; // remainder, not modulus

        let mut grow_indivs = ((indivs_per_depth as f32) / 2.0).floor() as i32;
        let mut full_indivs = ((indivs_per_depth as f32) / 2.0).ceil() as i32;

        for depth in 1..(max_init_depth+1) {
            // if at the last depth group, overwrite to include the poor remaining_indivs
            if depth == max_init_depth {
                grow_indivs = (((indivs_per_depth as f32) + (remaining_indivs as f32)) / 2.0).floor() as i32;
                full_indivs = (((indivs_per_depth as f32) + (remaining_indivs as f32)) / 2.0).ceil() as i32;
            }
            // fill depth group
            for _ in 0..full_indivs {p.core.push(Individual::full(depth, data.dims(), Some(data)));}
            for _ in 0..grow_indivs {p.core.push(Individual::grow(depth, data.dims(), Some(data)));}
        }
        p
    }

    pub fn fittest_extensive_search<'a>(&'a self) -> &'a Individual {
        let mut f = &self.core[0];
        for i in &self.core {
            if i.train() < f.train() {
                f = &i;
            }
        }
        f
    }

    /// There's possibly a closure for this
    pub fn tournament_select(&self, pool_size: usize) -> &Individual {
        let mut rng = thread_rng();
        let mut first_guy = &self.core[rng.gen_range(0, self.size())];
        for _ in 0..(pool_size - 1) {
            let new_guy = &self.core[rng.gen_range(0, self.size())];
            if new_guy.train() < first_guy.train() {
                first_guy = new_guy;
            }
        }
        first_guy
    }
}
