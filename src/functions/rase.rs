pub fn relative_average_spectral_error(estimated: &Vec<f64>, true_values: &Vec<f64>) -> f64 {
    let n = estimated.len();
    let mut mse = 0.0;
    let mut variance = 0.0;
    for i in 0..n {
        mse += (estimated[i] - true_values[i]).powi(2);
        variance += true_values[i].powi(2);
    }
    mse / n as f64 / variance / n as f64
}