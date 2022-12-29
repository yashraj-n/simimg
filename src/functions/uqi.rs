use crate::functions::utils::{mean, variance};


pub fn universal_quality_image_index(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    // Zip the two vectors together and map over the resulting iterator,
    // calculating the difference between each pair of elements
    let diffs: Vec<f64> = v1.iter().zip(v2.iter())
        .map(|(x, y)| (x - y).powi(2)).collect();

    // Calculate the mean and variance of the differences
    let mean_diff = mean(&diffs);
    let var_diff = variance(&diffs);

    // Calculate the UQI
    1.0 - mean_diff / var_diff
}



