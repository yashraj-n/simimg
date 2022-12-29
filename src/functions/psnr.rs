pub fn peak_signal_to_noise_ratio(original: &Vec<f64>, reconstructed: &Vec<f64>) -> f64 {
    // Zip the two vectors together and map over the resulting iterator,
    // calculating the difference between each pair of elements
    let diffs: Vec<f64> = original.iter().zip(reconstructed.iter())
        .map(|(x, y)| (x - y).powi(2)).collect();

    // Calculate the sum of the squares of the differences
    let sum: f64 = diffs.iter().sum();

    // Calculate the mean of the squares of the differences
    let mse = sum / (original.len() as f64);

    // Calculate the PSNR
    20.0 * mse.log10().abs()
}
