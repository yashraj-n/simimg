pub fn spectral_angle_mapper(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    let mut dot_product = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    for i in 0..a.len() {
        dot_product += a[i] * b[i];
        norm_a += a[i].powi(2);
        norm_b += b[i].powi(2);
    }
    norm_a = norm_a.sqrt();
    norm_b = norm_b.sqrt();
    let cosine = dot_product / (norm_a * norm_b);
    cosine.acos()
}
