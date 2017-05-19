extern crate rand;
extern crate rayon;

mod core;
use core::gp::GP;
use core::data::Data;
use core::multi_gp::Mgp;

fn main() {

    let ds = Data::new("yacht");

    let gp1 = GP::new_gp(ds.clone())
        .set_pop_size(25)
        .set_pool_size(3)
        .set_xo_rate(0.5);

    let gp2 = GP::new_gp(ds)
        .set_pop_size(25)
        .set_pool_size(3)
        .set_xo_rate(0.5);

    let mut mgp = Mgp::new().add_subpop(gp1)
                           .add_subpop(gp2);
    mgp.init();
    mgp.evolve_in_parallel(3, 30);
    // evolves two `GP` subpopulations in parallel for 3 turns of 30 generations.

}
