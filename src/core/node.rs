use rand::{thread_rng, Rng};
use core::utils::*;

#[derive(Debug, Clone)]
/// A node in a tree representation of an Individual.
/// `Constant(f32)` and `Input(usize)` represent terminal nodes;
/// the remaining, functional / operator nodes.
pub enum Node {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Cosine,
    LogFunction,
    Input(usize),
    Constant(f32),
}

impl Node {
    /// Returns how many arguments a node takes.
    pub fn arity(&self) -> usize {
        // usize
        match *self {
            Node::Input(_) => 0,
            Node::Constant(_) => 0,
            Node::Cosine | Node::LogFunction => 1,
            _ => 2,
        }
    }

    /// performs operation of node on the args.
    pub fn op(&self, args: Vec<Vec<f32>>) -> Vec<f32> {
        if self.arity() != args.len() {
            panic!("Number of args does not match node's arity.")
        }
        match *self {
            Node::Addition => add(args[0].to_vec(), args[1].to_vec()),
            Node::Subtraction => subtract(args[0].to_vec(), args[1].to_vec()),
            Node::Cosine => cosine(args[0].to_vec()),
            Node::Multiplication => multiply(args[0].to_vec(), args[1].to_vec()),
            Node::Division => divide(args[0].to_vec(), args[1].to_vec()),
            Node::LogFunction => logistic_function(args[0].to_vec()),
            _ => panic!("Tried to call Node::op() on a non-functional node."),
        }
    }

    /// Gets a random constant from `-1.0` to `1.0` in steps of `0.25`.
    /// Herein defined the constant set, a component of the terminal set.
    pub fn get_random_const() -> Node {
        let mut rng = thread_rng();
        let constant_set = [-1.0, -0.75, -0.5, -0.25, 0.0, 0.25, 0.5, 0.75, 1.0];
        let i = rng.gen_range(0, constant_set.len());
        Node::Constant(constant_set[i] as f32)
    }

    /// Gets a random functional node uniformly at random from the functional set
    /// defined in `enum Node`. LogFunction is considered to be only part of GSGP.
    pub fn get_random_functional() -> Node {
        let mut rng = thread_rng();
        match rng.gen_range(0, 5) { // for now
            0 => Node::Addition,
            1 => Node::Subtraction,
            2 => Node::Cosine,
            3 => Node::Multiplication,
            4 => Node::Division,
            _ => panic!("Non-existent Functional node."),
        }
    }

    pub fn get_random_input(dimensions: usize) -> Node {
        let mut rng = thread_rng();
        let i = rng.gen_range(0, dimensions);
        Node::Input(i)
    }
}
