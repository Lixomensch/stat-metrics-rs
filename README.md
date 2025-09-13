# stat-metrics-rs

`stat-metrics-rs` is a Rust library providing **common regression metrics** and **statistical utilities** for evaluating predictive models.

## Features

- Regression metrics:
  - **R²** and **Adjusted R²**
  - **MSE** (Mean Squared Error) and **RMSE** (Root Mean Squared Error)
  - **MAE** (Mean Absolute Error)
  - **MAPE** (Mean Absolute Percentage Error)
  - **RMSLE** (Root Mean Squared Logarithmic Error)
- Utility functions to support metric calculations:
  - `check_vectors` – validate slices before computation
  - `mean` – calculate arithmetic mean
  - `sum_of_squares` – compute sum of squared differences

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
stat-metrics-rs = { git = "https://github.com/Lixomensch/stat-metrics-rs.git" }
````

Then run:

```bash
cargo build
```

## Usage

You can import functions directly from the crate:

```rust
use stat_metrics_rs::*;

fn main() {
    let y_true = vec![3.0, -0.5, 2.0, 7.0];
    let y_pred = vec![2.5, 0.0, 2.0, 8.0];

    let r2_val = r_squared(&y_true, &y_pred);
    let adj_r2_val = adjusted_r_squared(&y_true, &y_pred, 2);
    let mse_val = mean_squared_error(&y_true, &y_pred);
    let rmse_val = root_mean_squared_error(&y_true, &y_pred);
    let mae_val = mean_absolute_error(&y_true, &y_pred);
    let mape_val = mean_absolute_percentage_error(&y_true, &y_pred);
    let rmsle_val = root_mean_squared_log_error(&y_true, &y_pred);

    println!("R²: {:.4}", r2_val);
    println!("Adjusted R²: {:.4}", adj_r2_val);
    println!("MSE: {:.4}", mse_val);
    println!("RMSE: {:.4}", rmse_val);
    println!("MAE: {:.4}", mae_val);
    println!("MAPE: {:.2}%", mape_val);
    println!("RMSLE: {:.4}", rmsle_val);
}
```

## License

MIT License – see [LICENSE](LICENSE) file for details.

