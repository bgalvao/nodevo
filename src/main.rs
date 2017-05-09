extern crate rand;
mod core;

use core::gp::GP;
use core::data::Data;

fn main() {

    let ds = Data::new("yacht");
    let mut gsgp = GP::new_gsgp(ds)
                    .set_pop_size(150)
                    .set_pool_size(5)
                    .set_xo_rate(0.0);

    gsgp.init_new_pop();
    gsgp.evolve(100);

}
