use std::io::BufReader;
use std::io::BufRead; // a trait of BufReader necessary for lines() method
use std::fs::File;
use std::io::Lines;

/// This struct assumes ONE! target output. Multiobjective optimization is not yet a feature.
#[derive(Debug)]
pub struct Data {
    // NOTE! Outputs to be predicted is assumed to be the last column!
    dimensions : usize,
    train : Vec<Vec<f32>>,
    test : Vec<Vec<f32>>,
    // to [j][i], e.g. test[0] gets the first variable for all instances ;)
}

impl Data {

    pub fn train(&self) -> &Vec<Vec<f32>> {&self.train}
    pub fn test(&self) -> &Vec<Vec<f32>> {&self.test}
    pub fn dims(&self) -> usize {self.dimensions}

    pub fn train_targets(&self) -> &Vec<f32> {&self.train[self.train.len() - 1]}
    pub fn test_targets(&self) -> &Vec<f32> {&self.test[self.test.len() - 1]}

    pub fn new(dataset: &'static str) -> Data {
        let train_d = "datasets/".to_string() + dataset + "/train.txt";
        let test_d = "datasets/".to_string() + dataset + "/test.txt";

        let train = Data::new_df(train_d);
        let dims = train.len() - 1; // due to transposition, each row is a variable instead of an instance.
        let test =  Data::new_df(test_d);
        Data{dimensions: dims, train: train, test: test}
    }

    fn new_df(filename : String) -> Vec<Vec<f32>> {
        let itr = Data::get_iterator(filename.as_str());
        let df = Data::transpose_array(Data::fill_array(itr));
        df
    }

    fn get_iterator(filepath : &str) -> Lines<BufReader<File>> {
        let f = File::open(filepath);
        let file; match f {
            Ok(k) => file = k,
            Err(e) => panic!(e),
        }
        let file = BufReader::new(file);
        let lines = file.lines();
        lines
    }

    fn fill_array(lines: Lines<BufReader<File>>) -> Vec<Vec<f32>> {
        let mut array : Vec<Vec<f32>> = vec![];
        for line in lines {
            let mut inst : Vec<f32> = vec![];
            for var in line.unwrap().split_whitespace() {
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
}
