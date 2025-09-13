use crate::utils::math::{check_vectors, mean, sum_of_squares};

/// Calculate the R-squared value from a set of true and predicted values.
///
/// R-squared is a measure of how well a model fits the data. It is
/// defined as 1 - (SS_res / SS_tot), where SS_res is the sum of squared
/// residuals and SS_tot is the total sum of squares.
///
/// This function takes two slices, `y_true` and `y_pred`, and returns the
/// R-squared value. The slices must have the same length.
///
/// # Example
/// ```
/// use stat_metrics_rs::regression::r2::r_squared;
///
/// let y_true = vec![1.0, 2.0, 3.0];
/// let y_pred = vec![1.1, 2.2, 3.3];
/// let r2_val = r_squared(&y_true, &y_pred);
/// println!("R²: {}", r2_val);
/// ```
pub fn r_squared(y_true: &[f64], y_pred: &[f64]) -> f64 {
    if !check_vectors(y_true, y_pred) {
        return f64::NAN;
    }

    let mean_y = mean(y_true);
    let ss_res: f64 = y_true
        .iter()
        .zip(y_pred.iter())
        .map(|(y, y_hat)| (y - y_hat).powi(2))
        .sum();
    let ss_tot = sum_of_squares(y_true, mean_y);

    if ss_tot.abs() < f64::EPSILON {
        return 1.0;
    }

    1.0 - ss_res / ss_tot
}

/// Calculate the Adjusted R-squared for multiple variables.
///
/// Adjusted R² penalizes adding irrelevant predictors:
/// 1 - (1 - R²) * (n - 1) / (n - p - 1)
///
/// # Arguments
/// * `y_true` - slice of true values
/// * `y_pred` - slice of predicted values
/// * `num_features` - number of features used in the model
///
/// # Returns
/// * `f64` - the adjusted R-squared value
///
/// # Example
/// ```
/// use stat_metrics_rs::regression::r2::adjusted_r_squared;
/// use stat_metrics_rs::regression::r2::r_squared;
///
/// let y_true = vec![3.0, -0.5, 2.0, 7.0];
/// let y_pred = vec![2.5, 0.0, 2.0, 8.0];
/// let num_features = 2;
/// let adj_r2_val = adjusted_r_squared(&y_true, &y_pred, num_features);
/// println!("Adjusted R²: {:.4}", adj_r2_val); // Example output: Adjusted R²: 0.9229
/// ```
pub fn adjusted_r_squared(y_true: &[f64], y_pred: &[f64], num_features: usize) -> f64 {
    let n = y_true.len();
    if n <= num_features + 1 {
        return f64::NAN;
    }
    let r2 = r_squared(y_true, y_pred);
    1.0 - (1.0 - r2) * (n as f64 - 1.0) / (n as f64 - num_features as f64 - 1.0)
}
