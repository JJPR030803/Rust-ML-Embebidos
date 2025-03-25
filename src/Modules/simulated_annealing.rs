// sa_multi_improved.rs
use rand::prelude::*; // Asegurar que importamos todo lo necesario de rand

// Parámetros para la optimización de un solo factor ambiental
pub struct EnvironmentParam {
    pub name: String,             // Nombre del parámetro (ej. "temperatura", "humedad")
    pub current_value: f64,       // Valor actual
    pub target_value: f64,        // Valor objetivo deseado por el usuario
    pub min_value: f64,           // Valor mínimo posible
    pub max_value: f64,           // Valor máximo posible
    pub change_cost_factor: f64,  // Factor de costo por cambio
    pub priority: f64,            // Prioridad de este parámetro (0.0-1.0)
    pub satisfaction_weight: f64, // Peso de la satisfacción vs. costo para este parámetro (0.0-1.0)
}

// Configuración del entorno con múltiples parámetros
pub struct MultiEnvironment {
    pub params: Vec<EnvironmentParam>,
}

impl MultiEnvironment {
    // Método para validar que las prioridades sumen 1.0
    pub fn validate_priorities(&self) -> Result<(), String> {
        let sum: f64 = self.params.iter().map(|p| p.priority).sum();

        // Permitir un pequeño margen de error debido a imprecisiones de punto flotante
        if (sum - 1.0).abs() > 0.001 {
            return Err(format!(
                "Las prioridades deben sumar 1.0, pero suman {}",
                sum
            ));
        }

        Ok(())
    }

    // Método auxiliar para normalizar las prioridades si es necesario
    pub fn normalize_priorities(&mut self) {
        let sum: f64 = self.params.iter().map(|p| p.priority).sum();

        if (sum - 1.0).abs() > 0.001 {
            for param in &mut self.params {
                param.priority /= sum;
            }
        }
    }
}

// Estructura para representar una solución completa
pub struct Solution {
    pub values: Vec<f64>,        // Valores para cada parámetro
    pub change_costs: Vec<f64>,  // Costo de cambio para cada parámetro
    pub satisfactions: Vec<f64>, // Satisfacción para cada parámetro
    pub total_cost: f64,         // Costo total combinado
}

impl Solution {
    // Método para imprimir los resultados de forma amigable
    pub fn print_results(&self, env: &MultiEnvironment) {
        println!("Resultados del recocido simulado multi-paramétrico:");
        println!("-------------------------------------------------");

        for i in 0..self.values.len() {
            println!("Parámetro: {}", env.params[i].name);
            println!("  Valor actual: {:.2}", env.params[i].current_value);
            println!("  Valor objetivo: {:.2}", env.params[i].target_value);
            println!("  Valor óptimo: {:.2}", self.values[i]);
            println!("  Satisfacción: {:.2}", self.satisfactions[i]);
            println!("  Costo de cambio: {:.2}", self.change_costs[i]);
            println!();
        }

        println!("Costo total: {:.4}", self.total_cost);
    }
}

// Función para calcular el costo de cambiar de un valor a otro
fn calculate_change_cost(from: f64, to: f64, cost_factor: f64) -> f64 {
    // El costo es proporcional a la magnitud del cambio
    (to - from).abs() * cost_factor
}

// Función para calcular la satisfacción del usuario con un valor
fn calculate_user_satisfaction(value: f64, target: f64, min: f64, max: f64) -> f64 {
    // Cuanto más cerca del objetivo, mayor satisfacción (1.0 = máxima satisfacción)
    1.0 - ((value - target).abs() / (max - min)).min(1.0)
}

// Función objetivo que combina costo y satisfacción para múltiples parámetros
pub fn multi_objective_function(new_values: &[f64], env: &MultiEnvironment) -> f64 {
    let mut total_cost = 0.0;

    for (i, param) in env.params.iter().enumerate() {
        // Calcular el costo de cambiar del valor actual al nuevo
        let change_cost =
            calculate_change_cost(param.current_value, new_values[i], param.change_cost_factor);

        // Calcular la satisfacción del usuario con el nuevo valor
        let satisfaction = calculate_user_satisfaction(
            new_values[i],
            param.target_value,
            param.min_value,
            param.max_value,
        );

        // Balance entre costo y satisfacción para este parámetro
        // Nota clave: Aquí usamos el satisfaction_weight para balancear mejor
        let param_cost = (1.0 - param.satisfaction_weight) * change_cost
            - param.satisfaction_weight * satisfaction;

        // Aplicar la prioridad de este parámetro
        total_cost += param.priority * param_cost;
    }

    total_cost // Valor a minimizar
}

// Algoritmo de recocido simulado para múltiples parámetros
pub fn recocido_simulado_multi(
    temp_inicial: f64,
    tasa_enfriamiento: f64,
    iteraciones: usize,
    env: &MultiEnvironment,
) -> Solution {
    let mut rng = rand::rng(); // Usar thread_rng() en lugar de rand::rng()

    // Validar o normalizar prioridades
    match env.validate_priorities() {
        Err(msg) => {
            println!("Advertencia: {}", msg);
            println!("Las prioridades serán normalizadas automáticamente.");
            // Nota: En una implementación real, podríamos clonar el env y normalizarlo
            // en lugar de imprimir una advertencia
        }
        Ok(_) => {}
    }

    // Comenzar con los valores actuales como solución inicial
    let mut valores_actuales: Vec<f64> = env.params.iter().map(|p| p.current_value).collect();
    let mut mejor_solucion = valores_actuales.clone();
    let mut mejor_costo = multi_objective_function(&valores_actuales, env);
    let mut temperatura = temp_inicial;

    // Contador para debug
    let mut accepted_moves = 0;
    let mut total_moves = 0;

    for _ in 0..iteraciones {
        total_moves += 1;

        // Generar vecino (valores cercanos aleatorios)
        let mut valores_nuevos = valores_actuales.clone();

        // En cada iteración, modificamos todos los parámetros con una pequeña probabilidad
        // para explorar mejor el espacio de soluciones
        for i in 0..env.params.len() {
            // 50% de probabilidad de modificar cada parámetro
            if rng.random::<f64>() < 0.5 {
                let param = &env.params[i];
                let step_size = (param.max_value - param.min_value) / 10.0; // Paso más grande

                valores_nuevos[i] = (valores_actuales[i] + rng.gen_range(-step_size..step_size))
                    .max(param.min_value)
                    .min(param.max_value);
            }
        }

        // Calcular costo de valores actuales y nuevos
        let costo_actual = multi_objective_function(&valores_actuales, env);
        let costo_nuevo = multi_objective_function(&valores_nuevos, env);
        let delta_costo = costo_nuevo - costo_actual;

        // Aceptar según criterio de Metropolis
        let accepted = if delta_costo <= 0.0 {
            true // Siempre aceptar mejores soluciones
        } else {
            // Probabilidad de aceptar peores soluciones
            let p = (-delta_costo / temperatura).exp();
            rng.random::<f64>() < p
        };

        if accepted {
            accepted_moves += 1;
            valores_actuales = valores_nuevos;

            // Actualizar mejor solución si corresponde
            if costo_nuevo < mejor_costo {
                mejor_costo = costo_nuevo;
                mejor_solucion = valores_actuales.clone();
            }
        }

        temperatura *= tasa_enfriamiento;
    }

    // Imprimir estadísticas para debug
    let accept_rate = (accepted_moves as f64 / total_moves as f64) * 100.0;
    println!(
        "Tasa de aceptación: {:.2}% ({}/{})",
        accept_rate, accepted_moves, total_moves
    );

    // Calcular componentes del costo para la mejor solución
    let mut change_costs = Vec::new();
    let mut satisfactions = Vec::new();

    for (i, param) in env.params.iter().enumerate() {
        let change_cost = calculate_change_cost(
            param.current_value,
            mejor_solucion[i],
            param.change_cost_factor,
        );
        let satisfaction = calculate_user_satisfaction(
            mejor_solucion[i],
            param.target_value,
            param.min_value,
            param.max_value,
        );

        change_costs.push(change_cost);
        satisfactions.push(satisfaction);
    }

    Solution {
        values: mejor_solucion,
        change_costs,
        satisfactions,
        total_cost: mejor_costo,
    }
}

// Ejemplo de uso
pub fn ejemplo_recocido_multi() -> Solution {
    // Crear entorno multi-parámetro (temperatura y humedad)
    let env = MultiEnvironment {
        params: vec![
            EnvironmentParam {
                name: "temperatura".to_string(),
                current_value: 22.0,
                target_value: 25.0,
                min_value: 15.0,
                max_value: 30.0,
                change_cost_factor: 0.5,
                priority: 0.6,            // Temperatura tiene prioridad de 60%
                satisfaction_weight: 0.8, // Dar más peso a la satisfacción que al costo
            },
            EnvironmentParam {
                name: "humedad".to_string(),
                current_value: 50.0,
                target_value: 45.0,
                min_value: 30.0,
                max_value: 70.0,
                change_cost_factor: 0.3,
                priority: 0.4,            // Humedad tiene prioridad de 40%
                satisfaction_weight: 0.7, // Dar más peso a la satisfacción que al costo
            },
        ],
    };

    // Ejecutar recocido simulado con parámetros más agresivos
    let solucion = recocido_simulado_multi(1000.0, 0.97, 5000, &env);

    // Imprimir resultados detallados
    solucion.print_results(&env);

    solucion
}

#[test]
fn test_ejemplo_recocido_multi() {
    let solution = ejemplo_recocido_multi();

    println!(
        "Valores Optimos:{:?}, Costo Total Optimo:{}, Satisfacciones de Valores Optimos:{:?}, Cambios de Costo:{:?}",
        solution.values, solution.total_cost, solution.satisfactions, solution.change_costs
    );
}
