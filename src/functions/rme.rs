pub fn rme(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let diffs: Vec<f64> = v1
        .iter()
        .zip(v2.iter())
        .map(|(x, y)| (x - y).powi(2))
        .collect();
    let sum: f64 = diffs.iter().sum();

    let mean: f64 = (sum / (v1.len() as f64)) as f64;

    mean.sqrt()
}
