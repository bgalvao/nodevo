use core::data::Data;
use core::node::Node;
use core::utils::rmse;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
/// The struct to represent an individual in a `Population`
pub struct Individual {
    core: Vec<Node>, // the author ponders: "to have or not to have an option"
    train_semantics: Option<Vec<f32>>,
    test_semantics: Option<Vec<f32>>,
    train: Option<f32>,
    test: Option<f32>,
    size: Option<usize>,
    depth: Option<usize>
}

impl Individual {

    // attribute access
    pub fn clone_core(&self) -> Vec<Node> {self.core.clone()}
    // returning self.core gives you a "cannot move out of borrowed content".
    // meaning that you're trying to pull it out "movingly".
    // this operation is used for grow() trees of max_depth 6, so cloning it should be cheap.

    pub fn train(&self) -> Option<f32> {self.train}
    pub fn test(&self) -> Option<f32> {self.test}

    pub fn size(&self) -> usize { // I think this will be better if returning a Result..
        if self.core.len() > 0 {
            return self.core.len()
        } else {
            self.size.expect("Size is not computed, while Vec<Node> is empty.")
        }
    }

    pub fn depth(&self) -> usize {
        self.depth.expect("Depth not computed.")
    }

    pub fn train_semantics(&self) -> Vec<f32> {
        self.train_semantics.clone().expect("Train data semantics not computed.")
    }

    pub fn test_semantics(&self) -> Vec<f32> {
        self.test_semantics.clone().expect("Test data semantics not computed.")
    }

    pub fn semantics(&self) -> (Vec<f32>, Vec<f32>) {
        (self.train_semantics(), self.test_semantics())
    }

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
            size: None,
            depth: None
        }
    }

    /// Generate a random individual using full method.
    pub fn full(max_depth: usize, data_ref: &Data) -> Individual {
        let mut i: Individual = Individual::new();
        i.inner_full(0, max_depth, data_ref.dims());
        i
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
    pub fn grow(max_depth: usize, data_ref: &Data) -> Individual {
        let mut i: Individual = Individual::new();
        i.inner_grow(0, max_depth, data_ref.dims());
        i
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

    fn get(&mut self, idx: usize) -> Node {
        self.core[idx].clone()
    }

    pub fn prepend_node(&mut self, node: Node) {
        self.core.insert(0, node);
    }

    pub fn compute_depth(&mut self) {
        let ref mut node_idx = 0; // Q: a reference to a mutable usize, or a mutable reference to a usize? A: FORMER
        let ref initial_depth = 0; // a reference to a usize
        self.depth = Some(0);
        self.inner_compute_depth(node_idx, initial_depth);
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

    pub fn compute_semantics(&mut self, data: &Data) {
        self.train_semantics = Some(self.output(0, data.train()));
        self.test_semantics = Some(self.output(0, data.test()));
    }

    pub fn evaluate(&mut self, data: &Data) {
        self.train = Some(rmse(&self.train_semantics(), data.train_targets()));
        self.test = Some(rmse(&self.test_semantics(), data.test_targets()));
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

pub mod variation {

    pub mod standard {

        use core::individual::Individual;
        use core::data::Data;
        extern crate rand; use rand::{thread_rng, Rng};

        pub fn crossover(p1: &Individual, p2: &Individual, data: &Data) -> Individual {
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

            offspring.compute_semantics(data);
            offspring.evaluate(data);
            offspring.size = Some(offspring.core.len());
            offspring.compute_depth();
            offspring
        }

        pub fn mutation(p1: &Individual, data: &Data) -> Individual {
            let mut offspring = Individual::new();
            let mut rng = thread_rng();

            let mutation_point = rng.gen_range(0, p1.size());
            let subnodes_p1 = p1.count_subtree_nodes(mutation_point);

            let p1_left_copy = p1.outer_left_copy(mutation_point);
            let mutation = Individual::grow(6, data);
            let p1_right_copy = p1.outer_right_copy(mutation_point + subnodes_p1);

            offspring.plug_in_core(p1_left_copy);
            offspring.plug_in_core(mutation.clone_core());
            offspring.plug_in_core(p1_right_copy);

            offspring.compute_semantics(data);
            offspring.evaluate(data);
            offspring.size = Some(offspring.core.len());
            offspring.compute_depth();
            offspring
        }

    }

    pub mod geometric_semantic {

        use std::cmp::max;
        use core::individual::Individual;
        use core::node::Node;
        use core::utils::{add, subtract, multiply};
        use core::data::Data;

        pub fn crossover(p1: &Individual, p2: &Individual, data: &Data) -> Individual {
            let p1_semantics = p1.semantics();
            let p2_semantics = p2.semantics();

            // the random tree is bounded to [0, 1], hence the log function node prepend
            // i.e.always bounded for gs crossover
            let mut r1 = Individual::grow(6, data);
            r1.prepend_node(Node::LogFunction);
            r1.compute_depth();
            r1.compute_semantics(data);
            let r1_semantics = r1.semantics();

            let train_semantics = gs_crossover_semantics(p1_semantics.0, p2_semantics.0, r1_semantics.0);
            let test_semantics = gs_crossover_semantics(p1_semantics.1, p2_semantics.1, r1_semantics.1);

            let mut offspring = Individual::new(); // empty in all kinds of info you can think of
            offspring.train_semantics = Some(train_semantics);
            offspring.test_semantics = Some(test_semantics);
            offspring.evaluate(data);
            offspring.size = Some(calc_xo_offspring_size(p1, p2, &r1));
            offspring.depth = Some(calc_xo_offspring_depth(p1, p2, &r1));
            offspring
        }

        fn gs_crossover_semantics(p1_semantics: Vec<f32>, p2_semantics: Vec<f32>, r1_semantics: Vec<f32>) -> Vec<f32> {
            // offspring semantics <- t1 * rb + (1 - rb) * t2
            let n = p1_semantics.len();
            add(multiply(p1_semantics.to_vec(), r1_semantics.to_vec()), multiply(subtract(vec![1f32; n], r1_semantics.to_vec()), p2_semantics.to_vec()))
        }

        fn calc_xo_offspring_size(p1: &Individual, p2: &Individual, r1: &Individual) -> usize {
            p1.size() + p2.size() + r1.size() * 2 + 5
        }

        fn calc_xo_offspring_depth(p1: &Individual, p2: &Individual, r1: &Individual) -> usize {
            let deepest = max(p1.depth(), p2.depth());
            max(deepest + 2, r1.depth() + 3 + 1)
        }

        pub fn mutation(p1: &Individual, data: &Data, mut_step:f32, bounded_mutation: bool) -> Individual {
            // Tm = T + ms (r1 - r2)
            let mut r1 = Individual::grow(6, data);
            let mut r2 = Individual::grow(6, data);
            if bounded_mutation {
                r1.prepend_node(Node::LogFunction);
                r2.prepend_node(Node::LogFunction);
            }
            r1.compute_depth();
            r1.compute_semantics(data);
            r2.compute_depth();
            r2.compute_semantics(data);

            let p1_semantics = p1.semantics();
            let r1_semantics = r1.semantics();
            let r2_semantics = r2.semantics();

            let train_semantics = gs_mutation_semantics(p1_semantics.0, r1_semantics.0, r2_semantics.0, mut_step);
            let test_semantics = gs_mutation_semantics(p1_semantics.1, r1_semantics.1, r2_semantics.1, mut_step);

            let mut offspring = Individual::new(); // empty in all kinds of info you can think of
            offspring.train_semantics = Some(train_semantics);
            offspring.test_semantics = Some(test_semantics);
            offspring.evaluate(data);
            offspring.size = Some(calc_mut_offspring_size(p1, &r1, &r2));
            offspring.depth = Some(calc_mut_offspring_depth(p1, &r1, &r2));
            offspring
        }

        fn gs_mutation_semantics(p1_sems: Vec<f32>, r1_sems: Vec<f32>, r2_sems: Vec<f32>, mut_step: f32) -> Vec<f32> {
            let len = r1_sems.len();
            add(p1_sems.to_vec(), multiply(vec![mut_step; len], subtract(r1_sems.to_vec(), r2_sems.to_vec())))
        }

        fn calc_mut_offspring_size(p1: &Individual, r1: &Individual, r2: &Individual) -> usize {
            p1.size() + 3 /* Plus, Constant, Mult */ + r1.size() + 1 + r2.size()
        }

        fn calc_mut_offspring_depth(p1: &Individual, r1: &Individual, r2: &Individual) -> usize {
            let max_depth = max(r1.depth(), r2.depth());
            max(max_depth + 3, p1.depth() + 1)
        }

    }


}
