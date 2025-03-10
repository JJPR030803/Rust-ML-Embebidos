use super::data_struct::Data;

/// Calcula los valores ponderados entre valores predichos y actuales
///
/// Parameters:
/// - actual: Los valores observados actuales
/// - predicted: Los valores predecidos por el modelo
/// - weights: Pesos opcionales (default si no es modificado)
///
/// Returns:
/// - El valor ponderado y metricas de analisis
pub fn calculate_weighted_comparison(
    actual: &[f64],
    predicted: &[f64],
    weights: Option<&[f64]>,
) -> WeightedComparisonResult {
    if actual.len() != predicted.len() {
        panic!("Actual and predicted values must have the same length");
    }

    let n = actual.len();

    // Use provided weights or default to equal weights
    let weights = match weights {
        Some(w) => {
            if w.len() != n {
                panic!("Weights must have the same length as values");
            }
            w.to_vec()
        }
        None => vec![1.0 / n as f64; n], // Equal weights
    };

    // Normalize weights to sum to 1.0 if they don't already
    let weight_sum: f64 = weights.iter().sum();
    let normalized_weights: Vec<f64> = weights.iter().map(|w| w / weight_sum).collect();

    // Calculate weighted values
    let mut weighted_values = Vec::with_capacity(n);
    for i in 0..n {
        weighted_values.push(normalized_weights[i] * (actual[i] + predicted[i]) / 2.0);
    }

    // Calculate error metrics
    let mut weighted_error = 0.0;
    let mut weighted_abs_error = 0.0;
    let mut weighted_squared_error = 0.0;

    for i in 0..n {
        let error = actual[i] - predicted[i];
        weighted_error += normalized_weights[i] * error;
        weighted_abs_error += normalized_weights[i] * error.abs();
        weighted_squared_error += normalized_weights[i] * error.powi(2);
    }

    // Calculate weighted RMSE
    let weighted_rmse = weighted_squared_error.sqrt();

    WeightedComparisonResult {
        weighted_values,
        original_weights: weights,
        normalized_weights,
        weighted_error,
        weighted_abs_error, // Weighted MAE
        weighted_rmse,
    }
}

/// Structure to hold the results of weighted comparison
#[derive(Debug)]
pub struct WeightedComparisonResult {
    pub weighted_values: Vec<f64>,
    pub original_weights: Vec<f64>,
    pub normalized_weights: Vec<f64>,
    pub weighted_error: f64,     // ME Ponderado
    pub weighted_abs_error: f64, // MAE Ponderado
    pub weighted_rmse: f64,      // RMSE Ponderado
}

/// Funcion para generar peso dependiendo de varios valores
/// Ejemplo:
/// -Equal
/// -Lineal
/// -Exponencial
/// -Distancia Inversa
pub fn generate_weights(len: usize, strategy: WeightingStrategy) -> Vec<f64> {
    match strategy {
        WeightingStrategy::Equal => vec![1.0 / len as f64; len],

        WeightingStrategy::Linear => {
            // Linear weights increasing with index (more weight to recent values)
            let sum = (len * (len + 1)) / 2;
            (1..=len).map(|i| i as f64 / sum as f64).collect()
        }

        WeightingStrategy::Exponential(alpha) => {
            // Exponential weights with decay factor alpha
            let mut weights = Vec::with_capacity(len);
            let mut sum = 0.0;

            for i in 0..len {
                let weight = (1.0 - alpha).powf(len as f64 - 1.0 - i as f64);
                weights.push(weight);
                sum += weight;
            }

            // Normalize
            weights.iter().map(|w| w / sum).collect()
        }

        WeightingStrategy::InverseDistance(distances) => {
            if distances.len() != len {
                panic!("Distances must have the same length as values");
            }

            let weights: Vec<f64> = distances
                .iter()
                .map(|&d| if d < 1e-10 { 1e10 } else { 1.0 / d })
                .collect();

            let sum: f64 = weights.iter().sum();
            weights.iter().map(|w| w / sum).collect()
        }
    }
}

/// Enum para opciones de peso
#[derive(Debug)]
pub enum WeightingStrategy {
    Equal,                     // Todos los pesos son iguales (se ve naranja por el todo)
    Linear,                    // Pesos incrementales lineales
    Exponential(f64),          // Pesos exponenciales con factor decayente
    InverseDistance(Vec<f64>), // Pesos calculados en la distancia inversa
}
