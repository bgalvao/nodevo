extern crate rand;
mod core;

//use core::individual::Individual;
//use core::node::Node;
//use core::population::Population;
use core::gp::GP;
use core::data::Data;

fn main() {

    let ds = Data::new("yacht");

    let mut gp = GP::default_new(ds);
    gp.init();
    gp.evolve(100);

}
