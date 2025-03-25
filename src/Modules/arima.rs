use nalgebra::{DMatrix, DVector};
use statrs::distribution::{ContinuousCDF, Normal};
use std::error::Error;

// Differencing function for the time series
fn difference(series: &[f64], order: usize) -> Vec<f64> {
    if order == 0 {
        return series.to_vec();
    }

    let diff_series: Vec<f64> = series.windows(2).map(|w| w[1] - w[0]).collect();
    if order == 1 {
        return diff_series;
    }

    difference(&diff_series, order - 1)
}

// Auto-regressive model estimation
fn fit_ar(series: &[f64], p: usize) -> Vec<f64> {
    // Create matrices for linear regression
    let n = series.len() - p;

    let mut y = Vec::with_capacity(n);
    let mut x = Vec::with_capacity(n * p);

    for i in p..series.len() {
        y.push(series[i]);

        for j in 0..p {
            x.push(series[i - j - 1]);
        }
    }

    let y_vec = DVector::from_vec(y);
    let x_mat = DMatrix::from_vec(n, p, x);

    // Solve linear equation to get AR coefficients
    match x_mat.transpose() * &x_mat {
        mat if mat.is_invertible() => {
            let x_inv = mat.try_inverse().unwrap();
            let coeffs = x_inv * (x_mat.transpose() * y_vec);
            coeffs.as_slice().to_vec()
        }
        _ => {
            // Fallback if matrix is not invertible
            vec![0.0; p]
        }
    }
}

// Moving average model estimation
fn fit_ma(residuals: &[f64], q: usize) -> Vec<f64> {
    // Simplified MA estimation using correlation
    let mut coeffs = vec![0.0; q];

    if residuals.len() <= q {
        return coeffs;
    }

    let mean = residuals.iter().sum::<f64>() / residuals.len() as f64;
    let variance =
        residuals.iter().map(|&r| (r - mean).powi(2)).sum::<f64>() / residuals.len() as f64;

    for i in 0..q {
        if i < residuals.len() - 1 {
            let lag_covariance = residuals
                .windows(2)
                .map(|w| (w[0] - mean) * (w[1] - mean))
                .sum::<f64>()
                / (residuals.len() - 1) as f64;
            coeffs[i] = lag_covariance / variance;
        }
    }

    coeffs
}

// ARIMA model struct
pub struct ArimaModel {
    pub ar_coeffs: Vec<f64>,
    pub ma_coeffs: Vec<f64>,
    pub d_order: usize,
    mean: f64,
    std_dev: f64,
    original_series: Vec<f64>,
    differenced_series: Vec<f64>,
    residuals: Vec<f64>,
}

impl ArimaModel {
    // Fit ARIMA(p,d,q) model
    pub fn fit(
        data: &[Data],
        field: &str,
        p: usize,
        d: usize,
        q: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let series = extract_series(data, field);

        // Check if we have enough data
        if series.len() <= p + d + q {
            return Err("Not enough data points for the specified ARIMA model".into());
        }

        // Apply differencing
        let differenced = difference(&series, d);

        // Calculate mean and standard deviation of differenced series
        let mean = differenced.iter().sum::<f64>() / differenced.len() as f64;
        let variance =
            differenced.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / differenced.len() as f64;
        let std_dev = variance.sqrt();

        // Fit AR model to get coefficients
        let ar_coeffs = fit_ar(&differenced, p);

        // Calculate residuals
        let mut residuals = Vec::with_capacity(differenced.len() - p);
        for i in p..differenced.len() {
            let mut predicted = mean;
            for j in 0..p {
                predicted += ar_coeffs[j] * (differenced[i - j - 1] - mean);
            }
            residuals.push(differenced[i] - predicted);
        }

        // Fit MA model to residuals
        let ma_coeffs = fit_ma(&residuals, q);

        Ok(ArimaModel {
            ar_coeffs,
            ma_coeffs,
            d_order: d,
            mean,
            std_dev,
            original_series: series,
            differenced_series: differenced,
            residuals,
        })
    }

    // Forecast future values
    pub fn forecast(&self, steps: usize) -> Vec<f64> {
        let p = self.ar_coeffs.len();
        let q = self.ma_coeffs.len();

        // Create forecasted differenced series
        let mut forecasted = Vec::with_capacity(steps);
        let mut extended_residuals = self.residuals.clone();

        // Get most recent values for forecasting
        let recent_diff_values: Vec<f64> = self
            .differenced_series
            .iter()
            .rev()
            .take(p)
            .cloned()
            .collect();

        // Forecast each step
        for _ in 0..steps {
            let mut forecast_value = self.mean;

            // Add AR component
            for j in 0..p {
                if j < recent_diff_values.len() {
                    forecast_value += self.ar_coeffs[j] * (recent_diff_values[j] - self.mean);
                }
            }

            // Add MA component
            for j in 0..q {
                if j < extended_residuals.len() {
                    forecast_value +=
                        self.ma_coeffs[j] * extended_residuals[extended_residuals.len() - 1 - j];
                }
            }

            // Add new forecasted value and assume zero for new residual
            forecasted.push(forecast_value);
            extended_residuals.push(0.0);
        }

        // Invert differencing to get original scale forecasts
        let mut result = forecasted;

        // Inverse differencing (simplified approach)
        for _ in 0..self.d_order {
            let last_original = self.original_series.last().unwrap_or(&0.0);
            for i in 0..result.len() {
                if i == 0 {
                    result[i] += last_original;
                } else {
                    result[i] += result[i - 1];
                }
            }
        }

        result
    }

    // Calculate AIC (Akaike Information Criterion) for model selection
    pub fn aic(&self) -> f64 {
        let n = self.residuals.len() as f64;
        let k = (self.ar_coeffs.len() + self.ma_coeffs.len() + 1) as f64; // +1 for variance

        // Sum of squared residuals
        let rss = self.residuals.iter().map(|r| r.powi(2)).sum::<f64>();

        // Log-likelihood (simplified)
        let log_likelihood = -0.5 * n * (1.0 + (2.0 * std::f64::consts::PI).ln() + (rss / n).ln());

        // AIC formula
        -2.0 * log_likelihood + 2.0 * k
    }
}

// Example usage function
pub fn analyze_time_series(filepath: &str) -> Result<(), Box<dyn Error>> {
    // Read data
    let data = read_csv(filepath)?;

    // Fit ARIMA model
    let arima_model = ArimaModel::fit(&data, "Temperature", 2, 1, 1)?;

    // Print model info
    println!("ARIMA Model Parameters:");
    println!("AR coefficients: {:?}", arima_model.ar_coeffs);
    println!("MA coefficients: {:?}", arima_model.ma_coeffs);
    println!("Mean of differenced series: {}", arima_model.mean);
    println!("AIC: {}", arima_model.aic());

    // Generate forecast
    let forecast = arima_model.forecast(5);
    println!("5-step forecast: {:?}", forecast);

    Ok(())
}
