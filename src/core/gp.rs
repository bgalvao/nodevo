extern crate rand;
use rand::{thread_rng, Rng};

use core::population::Population;
use core::individual::Individual;
use core::data::Data;

/// Phenotypic search component of the algorithm
enum Selection {
    Tournament,
    //FitnessProportionate, // TODO
    //Rank, // TODO
}

/* If later I implement various types of crossovers and mutations even for an ordinary GP,
 might have to change this for an enum.. */
trait Genotypic {
    fn crossover(p1: &Individual, p2: &Individual, data: &Data) -> Individual;
    fn mutation(p1: &Individual, data: &Data) -> Individual;
}

pub struct GP {
    data: Data,
    pop: Population, // has to hold state
    crossover_probability: f32,
    pop_size: usize,
    //elites: Some<usize>, // TODO
    pool_size: usize,
    depth_limit: Option<usize>,
    selection_method: Selection,
}

impl GP {

    pub fn default_new(dataset: Data) -> GP {
        GP {
            data: dataset,
            pop: Population::new(),
            crossover_probability: 0.9,
            pop_size: 100,
            pool_size: 4,
            depth_limit: None,
            selection_method: Selection::Tournament,
        }
    }

    pub fn init(&mut self) {
        if self.pop.is_empty() {
            self.pop = Population::new_rhh(self.pop_size, 6, &self.data); // eval individuals here?
        }
    }

    fn select(&self) -> &Individual {
        match self.selection_method {
            Selection::Tournament => self.pop.tournament_select(self.pool_size),
            //_ => panic!("SORRY: This selection method not yet implemented. Please opt for Selection::Tournament."),
        }
    }

    fn print_state(&self) {
        let f = self.pop.fittest_extensive_search();
        println!("----------------------");
        println!("train:\t{:?}", f.train().unwrap());
        println!("test:\t{:?}", f.test().unwrap());
        println!("size:\t{:?}", f.size());
        println!("depth:\t{:?}\n\n", f.depth().unwrap());
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
                    offspring = GP::crossover(p1, p2, &self.data);
                } else {
                    offspring = GP::mutation(p1, &self.data);
                }

                match self.depth_limit { // apply depth limit if present
                    None => {offspring_pop.add_individual(offspring);},
                    Some(limit) => {
                        // I derived Clone just because of this??
                        if offspring.depth().unwrap() > limit {offspring_pop.add_individual(p1.clone());}
                        else {offspring_pop.add_individual(offspring);}
                    },
                }
            }
            self.pop = offspring_pop;
            self.print_state();
        }
    }

}

impl Genotypic for GP {

    fn crossover(p1: &Individual, p2: &Individual, data: &Data) -> Individual {
        let mut offspring = Individual::new();
        let mut rng = thread_rng();

        let xo_point_p1 = rng.gen_range(0, p1.size());
        let xo_point_p2 = rng.gen_range(0, p2.size());

        let subnodes_p1 = p1.count_subtree_nodes(xo_point_p1);
        let subnodes_p2 = p2.count_subtree_nodes(xo_point_p2);

        let p1_left_copy = p1.outer_left_copy(xo_point_p1);
        let p2_subtree_copy = p2.copy_subtree(xo_point_p2, subnodes_p2);
        let p1_right_copy = p1.outer_right_copy(xo_point_p1 + subnodes_p1);

        offspring.plug_in_core(p1_left_copy);
        offspring.plug_in_core(p2_subtree_copy);
        offspring.plug_in_core(p1_right_copy);

        offspring.evaluate(data);
        offspring
    }

    fn mutation(p1: &Individual, data: &Data) -> Individual {
        let mut offspring = Individual::new();
        let mut rng = thread_rng();

        let mutation_point = rng.gen_range(0, p1.size());
        let subnodes_p1 = p1.count_subtree_nodes(mutation_point);

        let p1_left_copy = p1.outer_left_copy(mutation_point);
        let mutation = Individual::grow(6, data.dims(), None);
        let p1_right_copy = p1.outer_right_copy(mutation_point + subnodes_p1);

        offspring.plug_in_core(p1_left_copy);
        offspring.plug_in_core(mutation.clone_core());
        offspring.plug_in_core(p1_right_copy);

        offspring.evaluate(data);
        offspring
    }

}
