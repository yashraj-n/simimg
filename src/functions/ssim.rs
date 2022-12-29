use crate::functions::utils::{mean, variance};

pub fn structural_similarity_index(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    // Zip the two vectors together and map over the resulting iterator,
    // calculating the difference between each pair of elements
    let diffs: Vec<f64> = v1.iter().zip(v2.iter())
        .map(|(x, y)| (x - y).powi(2)).collect();

    // Calculate the mean and variance of the differences
    let var_diff = variance(&diffs);

    // Calculate the mean and variance of the values in each vector
    let mean1 = mean(v1);
    let var1 = variance(v1);
    let mean2 = mean(v2);
    let var2 = variance(v2);

    // Calculate the SSIM
    (2.0 * mean1 * mean2 + 1.0) * (2.0 * var_diff + 1.0) / ((mean1.powi(2) + mean2.powi(2) + 1.0) * (var1 + var2 + 1.0))
}

