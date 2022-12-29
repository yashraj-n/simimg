use super::utils::mean;

pub fn spatial_correlation_coefficient(x: &Vec<f64>, y:&Vec<f64>) -> f64 {
    let n = x.len();
    let mean_x = mean(x);
    let mean_y = mean(y);

    let mut sum_xy = 0.0;
    let mut sum_x_squared = 0.0;
    let mut sum_y_squared = 0.0;
    for i in 0..n {
        sum_xy += (x[i] - mean_x) * (y[i] - mean_y);
        sum_x_squared += (x[i] - mean_x).powi(2);
        sum_y_squared += (y[i] - mean_y).powi(2);
    }

    let std_x = (sum_x_squared / (n - 1) as f64).sqrt();
    let std_y = (sum_y_squared / (n - 1) as f64).sqrt();

    sum_xy / (std_x * std_y * (n - 1) as f64)
}
