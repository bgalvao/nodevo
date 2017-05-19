use core::gp::GP;
use core::individual::Individual;
use rayon::prelude::*;

pub struct Mgp {
    core: Vec<GP>,
    pop_size: usize,
}

impl Mgp {
    /// Returns a blank `Mgp` to be configured
    pub fn new() -> Mgp {
        Mgp {
            core: vec![],
            pop_size: 0,
        }
    }

    /// Adds a `GP` subpopulation.
    pub fn add_subpop(mut self, s: GP) -> Mgp {
        self.core.push(s);
        self
    }

    /// Initializes `Mgp` by calling initialization on its subpopulations
    ///
    /// Also, it sets the size of `Mgp` to the total of its constituent subpopulations.
    pub fn init(&mut self) {
        let total_size = self.core.par_iter().map(|gp| gp.pop().size()).sum();
        self.pop_size = total_size;
        self.core.par_iter_mut().for_each(|gp| gp.init_new_pop());
    }

    /// Evolves all the subpopulations for the specified number of `turns`
    /// where each turn lasts `gens_per_turn` generations. At the end of each turn,
    /// except for the last, a migration takes place (under a best-to-worst policy).
    /// All subpopulations are evolved in parallel and synchronize at migration instants.
    pub fn evolve_in_parallel(&mut self, turns: u8, gens_per_turn: usize) {
        for _ in 1..turns {
            // evolve for a while and then stop
            self.core
                .par_iter_mut()
                .for_each(|gp| gp.evolve(gens_per_turn));
            {
                // collect migrants and migrate in a ring pattern
                // we have subpops A, B and C. So migration goes like
                // A -> B; B -> C; and C -> A
                let migrants: Vec<Vec<Individual>> =
                    self.core.par_iter().map(|gp| gp.pop().clone_k_best(4)).collect();

                migrants
                    .par_iter()
                    .take(migrants.len() - 1)
                    .zip(self.core.par_iter_mut().skip(1))
                    .for_each(|(mgrnts, gp)| gp.pop_mut().add_individuals(mgrnts.to_vec()));

                // last migration
                self.core[0]
                    .pop_mut()
                    .add_individuals(migrants[migrants.len() - 1].to_vec());
            }
            // re-adjust subpopulation sizes.
            self.core.par_iter_mut().for_each(|gp| gp.clean());
        }
        // evolve for the last time without migration
        self.core
            .par_iter_mut()
            .for_each(|gp| gp.evolve(gens_per_turn));
        // only returns best individual with some other method!
    }
}
/*

let mut mgp = vec![gp1, gp2];

mgp.par_iter_mut()
   .for_each(|gp| gp.evolve(25)); // works until here.
{
let migrants: Vec<Vec<Individual>> = mgp.iter()
                                    .map(|gp| gp.pop().clone_k_best(4))
                                    .collect();

mgp.par_iter_mut()
   .zip(migrants.par_iter())
   .for_each(|(gp, migrants)| gp.pop_mut().add_individuals(migrants.to_vec()));
}

// par iter mut to clean population.
mgp.par_iter_mut().for_each(|gp| gp.clean());

*/
