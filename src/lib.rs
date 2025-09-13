//! stat-metrics-rs
//!
//! `stat-metrics-rs` is a Rust library providing **common regression metrics**
//! and **statistical utilities** for evaluating predictive models.
//!
//! # Features
//!
//! - Regression metrics:
//!     - **R²** and **Adjusted R²**
//!     - **MSE** (Mean Squared Error) and **RMSE** (Root Mean Squared Error)
//!     - **MAE** (Mean Absolute Error)
//!     - **MAPE** (Mean Absolute Percentage Error)
//!     - **RMSLE** (Root Mean Squared Logarithmic Error)
//! - Utility functions to support metric calculations

pub mod regression;
pub mod utils;

// Re-export regression metrics for simplified import
pub use regression::mae::mean_absolute_error;
pub use regression::mape::mean_absolute_percentage_error;
pub use regression::mse::{mean_squared_error, root_mean_squared_error};
pub use regression::r2::{adjusted_r_squared, r_squared};
pub use regression::rmsle::root_mean_squared_log_error;

// Re-export utility functions
pub use utils::math::{check_vectors, mean, sum_of_squares};
