use crate::utils::math::check_vectors;

/// Calculate the Root Mean Squared Logarithmic Error (RMSLE) between true and predicted values.
///
/// RMSLE is the square root of the average squared logarithmic differences between
/// actual and predicted values. It is useful when targets have exponential growth
/// or wide-ranging values.
///
/// # Arguments
/// * `y_true` - slice of true values
/// * `y_pred` - slice of predicted values
///
/// # Returns
/// * `f64` - the root mean squared logarithmic error
///
/// # Example
/// ```
/// use stat_metrics_rs::regression::rmsle::root_mean_squared_log_error;
///
/// let y_true = vec![3.0, 5.0, 2.0, 7.0];
/// let y_pred = vec![2.5, 5.0, 2.0, 8.0];
/// let rmsle_val = root_mean_squared_log_error(&y_true, &y_pred);
/// println!("RMSLE: {:.4}", rmsle_val); // Example output: RMSLE: 0.1438
/// ```
pub fn root_mean_squared_log_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    if !check_vectors(y_true, y_pred) {
        return f64::NAN;
    }

    let n = y_true.len() as f64;
    let sum_log = y_true
        .iter()
        .zip(y_pred.iter())
        .map(|(y, y_hat)| ((y + 1.0).ln() - (y_hat + 1.0).ln()).powi(2))
        .sum::<f64>();

    (sum_log / n).sqrt()
}
