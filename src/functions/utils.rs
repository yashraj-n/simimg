pub fn mean(values: &Vec<f64>) -> f64 {
    values.iter().sum::<f64>() / (values.len() as f64)
}

pub fn variance(values: &Vec<f64>) -> f64 {
    let mean = mean(values);
    values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (values.len() as f64 - 1.0)
}