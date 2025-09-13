use crate::utils::math::check_vectors;

/// Calculate the Mean Squared Error (MSE) between true and predicted values.
///
/// MSE is the average of the squared differences between actual and predicted values.
/// It is defined as (1/n) * Σ(y_true - y_pred)^2.
///
/// # Arguments
/// * `y_true` - slice of true values
/// * `y_pred` - slice of predicted values
///
/// # Returns
/// * `f64` - the mean squared error
///
/// # Example
/// ```
/// use stat_metrics_rs::regression::mse::mean_squared_error;
///
/// let y_true = vec![3.0, -0.5, 2.0, 7.0];
/// let y_pred = vec![2.5, 0.0, 2.0, 8.0];
/// let mse_val = mean_squared_error(&y_true, &y_pred);
/// println!("MSE: {:.4}", mse_val); // Output: MSE: 0.3750
/// ```
pub fn mean_squared_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    if !check_vectors(y_true, y_pred) {
        return f64::NAN;
    }

    let n = y_true.len() as f64;
    y_true
        .iter()
        .zip(y_pred.iter())
        .map(|(y, y_hat)| (y - y_hat).powi(2))
        .sum::<f64>()
        / n
}

/// Calculate the Root Mean Squared Error (RMSE) between true and predicted values.
///
/// RMSE is the square root of the Mean Squared Error (MSE), giving the error
/// in the same units as the target variable.
///
/// # Arguments
/// * `y_true` - slice of true values
/// * `y_pred` - slice of predicted values
///
/// # Returns
/// * `f64` - the root mean squared error
///
/// # Example
/// ```
/// use stat_metrics_rs::regression::mse::root_mean_squared_error;
///
/// let y_true = vec![3.0, -0.5, 2.0, 7.0];
/// let y_pred = vec![2.5, 0.0, 2.0, 8.0];
/// let rmse_val = root_mean_squared_error(&y_true, &y_pred);
/// println!("RMSE: {:.4}", rmse_val); // Output: RMSE: 0.6124
/// ```
pub fn root_mean_squared_error(y_true: &[f64], y_pred: &[f64]) -> f64 {
    mean_squared_error(y_true, y_pred).sqrt()
}
