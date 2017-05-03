use rand::{thread_rng, Rng};

use core::data::Data;
use core::node::Node;
use core::utils::rmse;

#[derive(Debug, Clone)]
/// The struct to represent an individual in a `Population`
pub struct Individual {
    /// The attribute `core` holds the mathematical function composed by nodes.
    /// The core is the tree structure representation of an `Individual`.
    core: Vec<Node>,
    train_semantics: Option<Vec<f32>>,
    test_semantics: Option<Vec<f32>>,
    train: Option<f32>,
    test: Option<f32>,
    depth: Option<usize>
}

impl Individual {

    // attribute access
    pub fn clone_core(&self) -> Vec<Node> {self.core.clone()}
    // returning self.core gives you a "cannot move out of borrowed content".
    // meaning that you're trying to pull it out "movingly".
    // this operation is used for grow() trees of max_depth 6, so cloning it should be chead.

    pub fn train(&self) -> Option<f32> {self.train}
    pub fn test(&self) -> Option<f32> {self.test}
    pub fn size(&self) -> usize {self.core.len()}
    pub fn depth(&self) -> Option<usize> {self.depth}

    // TODO use this for geometric semantic genetic programming, otherwise it's dead code.
    //pub fn train_semantics(&self) -> &Option<Vec<f32>> {&self.train_semantics}
    //pub fn test_semantics(&self) -> &Option<Vec<f32>> {&self.test_semantics}

    /*
    The process of creating a new individual:
    - Create empty individual using Individual::new()
    - Fill at random individual using full() or grow() method
    - If these functions are provided with a dataset `data_option`, then they are evaluated:
    - semantics are computed
    - training and test errors are computed
    - depth is computed

    Otherwise they are considered subtrees to serve purpose only in crossover and mutation and there is no need
    to evaluate extensively.

    Either way, ultimate mutability of the individual is decided by the way you cast e.g.
    `let i = Individual::full()` or `let mut i = Individual::grow()`

    The methods are written in the order of the described process.

    */

	pub fn new() -> Individual {
        Individual{
            core: vec![],
            train_semantics: None,
            test_semantics: None,
            train: None,
            test: None,
            depth: None
        }
    }

    /// Generate a random individual using full method.
    /// If using this method to grow a subtree of a tree, i.e. performing mutation,
    /// then one does not care about its quality in training error, test error, etc, in which
    /// case `data_option: None` ought to be passed. Otherwise, full evaluation of the individual is performed
    /// via the `evaluate` method.
    pub fn full(max_depth: usize, data_dims: usize, data_option: Option<&Data>) -> Individual {
        let mut i: Individual = Individual::new();
        match data_option {
            Some(data_ref) => {
                i.inner_full(0, max_depth, data_dims);
                i.evaluate(data_ref);
                i
            },
            None => {
                // perhaps an Option never used.
                // does not collect Individual.train nor Individual.test
                i.inner_full(0, max_depth, data_dims);
                i
            },
        }
    }

    fn inner_full(&mut self, current_depth: usize, max_depth: usize, data_dims: usize) {
        let mut rng = thread_rng();
        if current_depth == max_depth {
            if rng.gen() {self.core.push(Node::get_random_const());}
            else {self.core.push(Node::get_random_input(data_dims));}
        } else {
            let n = Node::get_random_functional(); let a = n.arity();
            self.core.push(n); // n moved!
            for _child_node in 0..a {
                self.inner_full(current_depth+1, max_depth, data_dims);
            }
        }
    }

    /// Generate a random individual using grow method.
    /// If using this method to grow a subtree of a tree, i.e. performing mutation,
    /// then one does not care about its quality in training error, test error, etc, in which
    /// case `data_option: None` ought to be passed. Otherwise, full evaluation of the individual is performed
    /// via the `evaluate` method.
    pub fn grow(max_depth: usize, data_dims: usize, data_option: Option<&Data>) -> Individual {
        let mut i: Individual = Individual::new();
        match data_option {
            Some(data_ref) => {
                i.inner_grow(0, max_depth, data_dims);
                i.evaluate(data_ref);
                i
            },
            None => {
                // perhaps an Option never used.
                // does not collect Individual.train nor Individual.test
                i.inner_grow(0, max_depth, data_dims);
                i
            },
        }
    }

    fn inner_grow(&mut self, current_depth: usize, max_depth: usize, data_dims: usize) {
        let mut rng = thread_rng();
        if current_depth == max_depth {
            if rng.gen() {self.core.push(Node::get_random_const());}
            else {self.core.push(Node::get_random_input(data_dims));}
        } else {
            if rng.gen() {
                let n = Node::get_random_functional(); let a = n.arity();
                self.core.push(n);
                for _child_node in 0..a {
                    self.inner_grow(current_depth+1, max_depth, data_dims);
                }
            } else {
                // 50/50 gets a constant or an input node
                if rng.gen() {self.core.push(Node::get_random_const());}
                else {self.core.push(Node::get_random_input(data_dims));}
            }
        }
    }

    pub fn evaluate(&mut self, data: &Data) {
        self.train_semantics = Some(self.output(0, data.train()));
        self.test_semantics = Some(self.output(0, data.test()));

        let sems = self.train_semantics.clone(); // by cloning, you are not immutably borrowing self ;)
        self.train = Some(rmse(sems.unwrap(), data.train_targets().to_vec()));

        let sems = self.test_semantics.clone(); // by cloning, you are not immutably borrowing self ;)
        self.test = Some(rmse(sems.unwrap(), data.test_targets().to_vec()));

        self.compute_depth(); // and this guy can safely mutably borrow self ;)
        //println!("train: {:?} :::: test: {:?}", self.train, self.test);
    }

    fn get(&mut self, idx: usize) -> Node {
        self.core[idx].clone()
        // without the clone(), the compiler throws me a:
        // "cannot move out of indexed content". I.e. with move semantics,
        // trying to simply return self.core[idx] is equivalent to pulling out the
        // Node out of self.core! So in order not to pull it out ~ move!, we instead
        // return a clone of it.
    }

    fn output(&mut self, mut idx: usize, df: &Vec<Vec<f32>>) -> Vec<f32> {
        let ref node = self.get(idx);
        match node {
            &Node::Constant(val) => vec![val; df[0].len()],
            &Node::Input(j) => df[j].to_vec(), // a copy that can totally be consumed
            _ => {
                let mut args = vec![] as Vec<Vec<f32>>;
                for _child_node in 0..node.arity() {
                    idx += 1;
                    args.push(self.output(idx, df));
                }
                node.op(args)
            },
        }
    }

    pub fn compute_depth(&mut self) {
        match self.depth {
            None => {
                let ref mut node_idx = 0; // Q: a reference to a mutable usize, or a mutable reference to a usize? A: FORMER
                let ref initial_depth = 0; // a reference to a usize
                self.depth = Some(0);
                self.inner_compute_depth(node_idx, initial_depth);
            },
            Some(d) => println!("Depth is computed and is {:?}", d),
        }
    }

    fn inner_compute_depth(&mut self, idx: &mut usize, current_depth: &usize) {
        // println!("idx: {} ::: depth: {} ::: node: {:?}", idx, current_depth, self.core[*idx]);
        for _child_node in 0..self.core[*idx].arity() {
            *idx += 1;
            self.inner_compute_depth(idx, &(current_depth + 1));
        }
        if *current_depth > self.depth.unwrap() {
            self.depth = Some(*current_depth);
        }
    }

    // from this point onwards, individual is no longer mutable and is considered complete!
    // these are aids to perform standard crossover and standard mutation
    pub fn count_subtree_nodes(&self, starting_index: usize) -> usize {
        match self.core[starting_index] {
            Node::Constant(_) => 1,
            Node::Input(_) => 1,
            _ => {
                let mut subtree_nodes = 1;
                for _child_node in 0..self.core[starting_index].arity() {
                    subtree_nodes += self.count_subtree_nodes(starting_index + subtree_nodes);
                }
                subtree_nodes
            }
        }
    }

    pub fn outer_left_copy(&self, excluding_node_at: usize) -> Vec<Node> {
        let mut left_copy = vec![] as Vec<Node>;
        // TODO try to do this with a decorator
        for i in 0..excluding_node_at {
            left_copy.push(self.core[i].clone());
        }
        left_copy
    }

    pub fn outer_right_copy(&self, including_node_at: usize) -> Vec<Node> {
        let mut right_copy = vec![] as Vec<Node>;
        // TODO once again, try to optimize with a decorator!
        for i in including_node_at..self.core.len() {
            right_copy.push(self.core[i].clone());
        }
        right_copy
    }

    pub fn copy_subtree(&self, from_node: usize, subtree_nodes: usize) -> Vec<Node> {
        let mut subtree_copy = vec![] as Vec<Node>;
        // TODO you know what to do to optimize
        for i in from_node..(from_node + subtree_nodes) {
            subtree_copy.push(self.core[i].clone());
        }
        subtree_copy
    }

    pub fn plug_in_core(&mut self, nodes: Vec<Node>) {
        for node in nodes {self.core.push(node);}
    }

}
