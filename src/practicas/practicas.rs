fn practica8() -> Result<(), Box<dyn Error>> {
    // Ruta del archivo CSV con los datos
    let filepath = "sensor_data.csv";

    // Leer los datos del archivo CSV
    println!("Leyendo datos desde {}...", filepath);
    let data = read_csv(filepath)?;
    println!(
        "Datos cargados exitosamente: {} puntos de datos",
        data.len()
    );

    // Configuración de los parámetros del modelo ARIMA
    let p = 2; // Orden autoregresivo (AR)
    let d = 1; // Orden de diferenciación
    let q = 1; // Orden de media móvil (MA)

    // División de datos en entrenamiento y prueba
    let train_size = (data.len() as f64 * 0.8) as usize;
    let train_data = data[0..train_size].to_vec();
    let test_data = data[train_size..].to_vec();

    // Extraer valores reales de prueba para comparación
    let actual_values = extract_series(&test_data, "Temperature");

    // Ajustar el modelo ARIMA con los datos de entrenamiento
    println!(
        "Ajustando modelo ARIMA({}, {}, {}) para la temperatura...",
        p, d, q
    );

    match ArimaModel::fit(&train_data, "Temperature", p, d, q) {
        Ok(model) => {
            // Generar pronóstico para el período de prueba
            let steps = test_data.len();
            let predicted_values = model.forecast(steps);

            println!("\nEvaluación del modelo:");
            println!("Número de puntos de prueba: {}", steps);

            // Cálculo de métricas de error básicas
            let mut total_error = 0.0;
            let mut total_abs_error = 0.0;
            let mut total_squared_error = 0.0;

            for i in 0..steps {
                let error = actual_values[i] - predicted_values[i];
                total_error += error;
                total_abs_error += error.abs();
                total_squared_error += error.powi(2);
            }

            let me = total_error / steps as f64;
            let mae = total_abs_error / steps as f64;
            let rmse = (total_squared_error / steps as f64).sqrt();

            println!("Error medio (ME): {:.4}", me);
            println!("Error absoluto medio (MAE): {:.4}", mae);
            println!("Raíz del error cuadrático medio (RMSE): {:.4}", rmse);

            // Comparaciones ponderadas con diferentes estrategias
            println!("\nComparaciones ponderadas:");

            // 1. Pesos lineales (más peso a valores recientes)
            let linear_weights = generate_weights(steps, WeightingStrategy::Linear);
            let linear_result = calculate_weighted_comparison(
                &actual_values,
                &predicted_values,
                Some(&linear_weights),
            );
            println!("Ponderación lineal (valores recientes enfatizados):");
            println!("  MAE ponderado: {:.4}", linear_result.weighted_abs_error);
            println!("  RMSE ponderado: {:.4}", linear_result.weighted_rmse);

            // 2. Pesos exponenciales
            let exp_weights = generate_weights(steps, WeightingStrategy::Exponential(0.2));
            let exp_result = calculate_weighted_comparison(
                &actual_values,
                &predicted_values,
                Some(&exp_weights),
            );
            println!("Ponderación exponencial (alpha=0.2):");
            println!("  MAE ponderado: {:.4}", exp_result.weighted_abs_error);
            println!("  RMSE ponderado: {:.4}", exp_result.weighted_rmse);

            // 3. Valores ponderados combinados
            println!("\nValores combinados ponderados:");
            for i in 0..steps.min(5) {
                // Mostrar los primeros 5 puntos
                println!(
                    "Punto {}: Real={:.2}, Predicho={:.2}, Ponderado={:.2}",
                    i + 1,
                    actual_values[i],
                    predicted_values[i],
                    linear_result.weighted_values[i]
                );
            }

            // Usar los valores ponderados para lógica adicional
            let weighted_forecast = linear_result.weighted_values;
            println!("\nEl pronóstico ponderado está listo para su uso en procesos posteriores...");
        }
        Err(e) => {
            eprintln!("Error al ajustar el modelo ARIMA: {}", e);
        }
    }

    Ok(())
}

fn practica7() -> Result<(), Box<dyn Error>> {
    // Path to your CSV file
    let filepath = "sensor_data.csv";

    // Read data from CSV
    println!("Reading data from {}...", filepath);
    let data = read_csv(filepath)?;
    println!("Successfully loaded {} data points", data.len());

    // Configure ARIMA parameters
    let p = 2; // AR order
    let d = 1; // Differencing order
    let q = 1; // MA order

    // Fit ARIMA model for temperature
    println!("Fitting ARIMA({},{},{}) model for Temperature...", p, d, q);
    match ArimaModel::fit(&data, "Temperature", p, d, q) {
        Ok(model) => {
            // Print model information
            println!("\nModel Information:");
            println!("AR coefficients: {:?}", model.ar_coeffs);
            println!("MA coefficients: {:?}", model.ma_coeffs);
            println!("AIC: {:.4}", model.aic());

            // Generate forecast
            let steps = 10;
            let forecast = model.forecast(steps);
            println!("\n{}-step Temperature forecast:", steps);
            for (i, value) in forecast.iter().enumerate() {
                println!("Step {}: {:.2}°", i + 1, value);
            }

            // You can also fit a model for humidity
            println!("\nFitting ARIMA model for Humidity...");
            if let Ok(humidity_model) = ArimaModel::fit(&data, "Humidity", p, d, q) {
                let humidity_forecast = humidity_model.forecast(steps);
                println!("\n{}-step Humidity forecast:", steps);
                for (i, value) in humidity_forecast.iter().enumerate() {
                    println!("Step {}: {:.2}%", i + 1, value);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fitting ARIMA model: {}", e);
        }
    }

    Ok(())
}

fn practica9() -> Result<(), Box<dyn Error>> {
    //TODO practica 9

    Ok(())
}
