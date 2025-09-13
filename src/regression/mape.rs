use crate::utils::math::check_vectors;

/// Calculate the Mean Absolute Percentage Error (MAPE) between true and predicted values.
///
/// MAPE is the average of the absolute percentage differences between
/// actual and predicted values. It is expressed as a percentage.
///
/// # Arguments
/// * `y_true` - slice of true values
/// * `y_pred` - slice of predicted values
///
/// # Returns
/// * `f64` - the mean absolute percentage error
///
/// # Example
/// ```
/// use stat_metrics_rs::regression::mape::mean_absolute_percentage_error;
///
/// let y_true = vec![100.0, 200.0, 300.0];
/// let y_pred = vec![110.0, 190.0, 310.0];
/// let mape_val = mean_absolute_percentage_error(&y_true, &y_pred);
/// println!("MAPE: {:.2}%", mape_val); // Output: MAPE: 5.56%
/// ```
pub fn mean_absolute_percentage_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    if !check_vectors(y_true, y_pred) {
        return f64::NAN;
    }

    let n = y_true.len() as f64;
    let mut sum = 0.0;
    for (y, y_hat) in y_true.iter().zip(y_pred.iter()) {
        if y.abs() < f64::EPSILON {
            continue;
        }
        sum += ((y - y_hat) / y).abs();
    }
    sum / n * 100.0
}
