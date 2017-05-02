use std::io::BufReader;
use std::io::BufRead; // a trait of BufReader necessary for lines() method
use std::fs::File;
use std::io::Lines;

#[derive(Debug)]
pub struct Data {
    // NOTE! Outputs to be predicted is assumed to be the last column!
    dimensions : usize,
    train : Vec<Vec<f32>>,
    test : Vec<Vec<f32>>,
    // to [j][i], e.g. test[0] gets the first variable for all instances ;)
}

impl Data {

    pub fn new() -> Data {
        let train = Data::new_df("dataset/train1.txt");
        let test = Data::new_df("dataset/test1.txt");
        let dims = Data::get_dim("dataset/test1.txt");
        Data{dimensions: dims, train: train, test: test}
    }

    pub fn train(&self) -> &Vec<Vec<f32>> {&self.train}
    pub fn test(&self) -> &Vec<Vec<f32>> {&self.test}
    pub fn dims(&self) -> usize {self.dimensions}

    pub fn train_targets(&self) -> &Vec<f32> {&self.train[self.train.len() - 1]}
    pub fn test_targets(&self) -> &Vec<f32> {&self.test[self.test.len() - 1]}

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

    fn get_dim(filename : &'static str) -> usize {
        let mut itr = Data::get_iterator(filename);
        let dimensions : usize = itr.next().unwrap().unwrap().parse().unwrap();
        dimensions
    }
}
