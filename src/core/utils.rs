pub fn rmse(x: Vec<f32>, y: Vec<f32>) -> f32 {
    let mapper = x.iter().zip(y.iter()).map(|(x,y)| ((x-y) as f32).powi(2));
    let se: f32 = mapper.sum();
    (se / (x.len() as f32)).sqrt()
}

pub fn add(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    x.iter().zip(y.iter()).map(|(x,y)| x+y).collect() // note: the return spec is what coerces collect() to a Vec<f32>
}

pub fn subtract(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    x.iter().zip(y.iter()).map(|(x,y)| x-y).collect()
}

pub fn multiply(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    x.iter().zip(y.iter()).map(|(x,y)| x*y).collect()
}

pub fn cosine(x: Vec<f32>) -> Vec<f32> {
    x.iter().map(|x| x.cos()).collect()
}

pub fn divide(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    let lower_limit = 0.00001; // this can totally influence semantics!
    let protected_division = |(n,d): (&f32, &f32)| -> f32 { // numerator and denominator
        if d.abs() > lower_limit {return n / d;}
        else {return n / 1.0;}
    };
    x.iter().zip(y.iter()).map(protected_division).collect()
}
