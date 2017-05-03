pub fn rmse(x: Vec<f32>, y: Vec<f32>) -> f32 {
    if x.len() != y.len() {panic!("cannot rmse vectors of different length!");}
    let n = x.len() as f32;
    let mapper = x.into_iter().zip(y.into_iter()).map(|(x,y)| ((x-y) as f32).powi(2));
    let se: f32 = mapper.sum();
    (se / n).sqrt()
}

pub fn add(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    if x.len() != y.len() {panic!("cannot sum element-wise vectors of different length!");}
    else {x.into_iter().zip(y.into_iter()).map(|(x,y)| x+y).collect()}
}

pub fn subtract(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    if x.len() != y.len() {panic!("cannot subtract element-wise vectors of different length!");}
    else {x.into_iter().zip(y.into_iter()).map(|(x,y)| x-y).collect()}
}

pub fn multiply(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    if x.len() != y.len() {panic!("cannot multiply element-wise vectors of different length!");}
    else {x.into_iter().zip(y.into_iter()).map(|(x,y)| x*y).collect()}
}

pub fn cosine(x: Vec<f32>) -> Vec<f32> {
    x.into_iter().map(|x| x.cos()).collect()
}

pub fn divide(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    if x.len() != y.len() {panic!("cannot divide element-wise vectors of different length!");}
    let lower_limit = 0.00001; // this can totally influence semantics!
    let protected_division = |(n,d): (f32, f32)| -> f32 { // numerator and denominator
        if d.abs() > lower_limit {return n / d;}
        else {return n / 1.0;}
    };
    x.into_iter().zip(y.into_iter()).map(protected_division).collect()
}
