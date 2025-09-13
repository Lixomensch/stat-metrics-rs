use crate::utils::math::check_vectors;

/// Calculate the Mean Absolute Error (MAE) between true and predicted values.
///
/// MAE is the average of the absolute differences between actual and predicted values.
/// It is in the same units as the target variable.
///
/// # Arguments
/// * `y_true` - slice of true values
/// * `y_pred` - slice of predicted values
///
/// # Returns
/// * `f64` - the mean absolute error
///
/// # Example
/// ```
/// use stat_metrics_rs::regression::mae::mean_absolute_error;
///
/// let y_true = vec![3.0, -0.5, 2.0, 7.0];
/// let y_pred = vec![2.5, 0.0, 2.0, 8.0];
/// let mae_val = mean_absolute_error(&y_true, &y_pred);
/// println!("MAE: {:.4}", mae_val); // Output: MAE: 0.5000
/// ```
pub fn mean_absolute_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    if !check_vectors(y_true, y_pred) {
        return f64::NAN;
    }

    let n = y_true.len() as f64;
    y_true
        .iter()
        .zip(y_pred.iter())
        .map(|(y, y_hat)| (y - y_hat).abs())
        .sum::<f64>()
        / n
}
