extern crate rand;
mod core;

//use core::individual::Individual;
//use core::node::Node;
//use core::population::Population;
use core::gp::GP;
use core::data::Data;

fn main() {
    println!("\n\nIt compiled!\n\n");

    let ds = Data::new();
    let mut gp = GP::default_new(ds);
    gp.init();
    gp.evolve(100);


}
