/// Check if two slices are valid for metric calculations.
///
/// This function ensures that:
/// 1. Both slices have the same length.
/// 2. Neither slice is empty.
///
/// Returns `true` if both conditions are met, otherwise returns `false`.
///
/// # Arguments
/// * `y_true` - slice of true values
/// * `y_pred` - slice of predicted values
///
/// # Returns
/// * `bool` - whether the slices are valid for metric calculations
///
/// # Example
/// ```
/// use stat_metrics_rs::utils::math::check_vectors;
///
/// let y_true = vec![1.0, 2.0, 3.0];
/// let y_pred = vec![1.1, 2.1, 3.2];
/// assert!(check_vectors(&y_true, &y_pred));
///
/// let invalid_pred = vec![1.0, 2.0]; // different length
/// assert!(!check_vectors(&y_true, &invalid_pred));
/// ```
pub fn check_vectors(y_true: &[f64], y_pred: &[f64]) -> bool {
    !y_true.is_empty() && y_true.len() == y_pred.len()
}

/// Calculate the arithmetic mean (average) of a slice of numbers.
///
/// Returns `NaN` if the slice is empty.
///
/// # Arguments
/// * `values` - slice of `f64` values
///
/// # Returns
/// * `f64` - the mean of the values, or `NaN` if the slice is empty
///
/// # Example
/// ```
/// use stat_metrics_rs::utils::math::mean;
///
/// let values = vec![1.0, 2.0, 3.0, 4.0];
/// let avg = mean(&values);
/// assert_eq!(avg, 2.5);
///
/// let empty: Vec<f64> = vec![];
/// assert!(mean(&empty).is_nan());
/// ```
pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Calculate the sum of squares of the differences between each value and the mean.
///
/// This function is commonly used in regression metrics like R², MSE, and variance calculations.
///
/// # Arguments
/// * `values` - slice of `f64` values
/// * `mean` - the mean of the values
///
/// # Returns
/// * `f64` - the sum of squared differences
///
/// # Example
/// ```
/// use stat_metrics_rs::utils::math::sum_of_squares;
///
/// let values = vec![1.0, 2.0, 3.0, 4.0];
/// let mean_val = 2.5;
/// let ss = sum_of_squares(&values, mean_val);
/// assert_eq!(ss, 5.0); // (1-2.5)^2 + (2-2.5)^2 + (3-2.5)^2 + (4-2.5)^2 = 5
/// ```
pub fn sum_of_squares(values: &[f64], mean: f64) -> f64 {
    values.iter().map(|v| (v - mean).powi(2)).sum()
}
