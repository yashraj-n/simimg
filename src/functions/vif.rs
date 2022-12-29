use super::{rme::rme, utils::variance};

pub fn visual_information_fidelity(original: &Vec<f64>, reconstructed: &Vec<f64>) -> f64 {
    let mse = rme(original, reconstructed);
    let variance = variance(original);
    mse / variance
}
