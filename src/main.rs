use std::io::BufReader;
use std::io::BufRead; // a trait of BufReader necessary for lines() method
use std::fs::File;
use std::io::Lines;

#[derive(Debug)]
struct Data {
    // NOTE! Outputs to be predicted is assumed to be the last column!
    dimensions : i32,
    train : Vec<Vec<f32>>,
    test : Vec<Vec<f32>>,
    //  to [j][i], e.g. test[0] gets the first variable for all instances ;)
}

impl Data {

    fn get_iterator(filepath : &'static str) -> Lines<BufReader<File>> {
        let f = File::open(filepath).unwrap();
        let file = BufReader::new(f);
        let lines = file.lines();
        lines
    }

    fn fill_array(lines: Lines<BufReader<File>>) -> Vec<Vec<f32>> {
        let mut array : Vec<Vec<f32>> = vec![];
        for line in lines {
            let mut inst : Vec<f32> = vec![];
            for var in line.unwrap().split("\t") {
                let val : f32 = var.parse().unwrap();
                inst.push(val);
            }
            array.push(inst);
        }
        let array = array;
        array
    }

    fn transpose_array(array : Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        let mut result : Vec<Vec<f32>> = vec![];
        for j in 0..array[0].len() {
            // first collect column
            let mut column = vec![];
            for i in 0..array.len() {
                column.push(array[i][j]);
            }
            // then push column as a row into result
            result.push(column);
        }
        result
    }

    fn new_df(filename : &'static str) -> Vec<Vec<f32>> {
        let mut itr = Data::get_iterator(filename);
        itr.next(); itr.next();
        let df = Data::transpose_array(Data::fill_array(itr));
        df
    }

    fn get_dim(filename : &'static str) -> i32 {
        let mut itr = Data::get_iterator(filename);
        let dimensions : i32 = itr.next().unwrap().unwrap().parse().unwrap();
        dimensions
    }

    fn new() -> Data {
        let train = Data::new_df("dataset/train1.txt");
        let test = Data::new_df("dataset/test1.txt");
        let dims = Data::get_dim("dataset/test1.txt");
        Data{dimensions: dims, train: train, test: test}
    }
}

#[derive(Debug)]
enum Node {
    Add,
    Cosine,
    Cons{value: f32, length: i32}, // or maybe pass a ref to a memburh of Data?
    Input<'a>{data_ref: &'a Data, index: i32},
}

impl Node {

    fn print(&self) {
        match self {
            &Node::Add => print!("+"),
            &Node::Cosine => print!("cos"),
            &Node::Input{data_ref, index} => print!("X{}", index),
            &Node::Cons{value, length} => print!("C{}", value),
        }
    }

    fn arity(&self) -> u8 {
        match self {
            &Node::Add => 2u8,
            &Node::Cosine => 1u8,
            _ => 0u8,
        }
    }

    fn op(&self, args: Vec<Vec<f32>>) -> Vec<f32> {
        match self {
            &Node::Add => self.add(args),
            &Node::Cosine => self.cosine(args),
            &Node::Cons{value, length} => vec![value; length],
            &Node::Input{data_ref: dr, index: idx} => self.get_input(self, dr, idx),
        }
    }

    fn add(&self, args: Vec<Vec<f32>>) -> Vec<f32> {
        match args.len() {

            2 => { // hardcoded for now...
                let mut res = vec![];
                for i in 0..args[0].len() {
                    res.push(args[0][i] + args[1][i]);
                }
                res
            },
            _ => panic!("args length does not match arity of Node::Add: {}", Node::Add.arity()),
        }
    }

    fn cosine(args: Vec<Vec<f32>>) -> Vec<f32> {
        match args.len() {
            1 => {
                let mut res = vec![];
                for i in args.0 {
                    res.push(i.cos());
                }
                res
            },
            _ => panic!("args length does not match arity of Node::Cosine: {}", Node::Add.arity()),
        }
    }

    fn get_input(&self, data_ref: &Data, index: i32) -> Vec<f32> {
        if index == data_ref.len()-1 {
            panic!("Node::Input -> tried to return last column of dataset\nsurely the output variable!");
        } else {
            return data_ref[index];
        }
    }

}


struct Indiv {p: Vec<Node>}

impl Indiv {

    fn new() -> Indiv {
        let v = vec![] as Vec<Node>;
        Indiv{p: v}
    }

    /* base case -> terminal; else go deeper until finding base case */
    fn eval(&self) -> Vec<f32> {
        let mut idx: usize = 0;
        self.inner_eval(idx);
    }

    fn inner_eval(&self, idx: &mut usize) -> Vec<f32> {
        let &mut node = self.p[idx];
        match node {
            &Node::Input{data_ref, index} => node.get_input(data_ref, index),
            &Node::Cons{value, length} => vec![value; length],
            _ => {
                let mut args = vec![] as Vec<Vec<f32>>;
                for i in 0..node.arity() {
                    idx += 1;
                    args[i] = self.inner_eval(self, idx);
                }
                node.op()
            }
        }
    }

}



fn main() {

    let data = Data::new();


    let mut test_node = Node::Add;
    test_node.print();

    //test_node = Node::Cons;
    test_node.print();

    println!("{:?}", test_node);
}

struct Population {idis: Vec<Indiv>}
impl Population {fn get_best(&self) -> Indiv {}}

trait BaselineGP {}
trait GSGP : BaselineGP {}

struct GP {data: Data}

