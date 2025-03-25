use std::{f64::consts::E, vec};

use super::val_satisfaccion_individual::ValoresSatisfaccionIndividual;
use rand::{Rng, thread_rng, random};

pub struct Annealing {
   pub temperatura_inicial: f64,
   pub tasa_enfriamiento: f64,
   pub temperatura_minima: f64,
   pub iteraciones: usize,
   pub preferencias: ValoresSatisfaccionIndividual,
   pub show_progress: bool,
   pub delta: f64,
}

impl Annealing {
   pub fn new(
       temperatura_inicial: f64,
       tasa_enfriamiento: f64,
       temperatura_minima: f64,
       iteraciones: usize,
       preferencias: ValoresSatisfaccionIndividual,
       show_progress: bool,
       delta: f64
   ) -> Self {
       Annealing {
           temperatura_inicial,
           tasa_enfriamiento,
           temperatura_minima,
           iteraciones,
           preferencias,
           show_progress,
           delta,
       }
    }

    // Genera una solucion vecina solo modificando la temperatura y la satisfaccion
    fn generar_vecino(&self, solucion_actual: &ValoresSatisfaccionIndividual) -> ValoresSatisfaccionIndividual {
        let mut rng = rand::rng();

        // Aquui se modifica el rango de generacion de vecinos ej. (-2.0..2.0)
        let vecino_temp = (solucion_actual.temp_actual + rng.random_range(-2.0..2.0))
            .max(solucion_actual.minimo)
            .min(solucion_actual.maximo);
        //Lo mismo de arriba pero con la satisfaccion
        let vecino_satisfaccion = (solucion_actual.satisfaccion_actual + rng.random_range(-10.0..10.0))
            .max(0.0)
            .min(100.0);
        //Un nuevo objeto de satisfaccion con los valores modificados
        ValoresSatisfaccionIndividual::new(
            solucion_actual.minimo,
            solucion_actual.maximo,
            vecino_temp,
            solucion_actual.temp_objetivo,
            vecino_satisfaccion,
            solucion_actual.satisfaccion_objetivo,
            solucion_actual.costo_unitario,
            solucion_actual.peso_satisfaccion,
        )
    }

    // Probabilidad de aceptar la nueva solucion
    fn probabilidad_aceptacion(delta: f64, temperatura: f64) -> f64 {
        if delta < 0.0 {
            1.0  // Always accept improvements
        } else {
            (-delta / temperatura).exp()
        }
    }

    // Ejecuta el algoritmo de recocido simulado
    pub fn run(&self) -> ValoresSatisfaccionIndividual {
        let mut rng = rand::rng();

        let mut solucion_actual = self.preferencias;
        let mut mejor_solucion = solucion_actual;
        let mut temperatura = self.temperatura_inicial;

        // Track progression for debugging debug purposes
        let mut total_accepted = 0;
        let mut total_iterations = 0;

        while temperatura > self.temperatura_minima {
            for _ in 0..self.iteraciones {
                // Genera un vecino
                let vecino = self.generar_vecino(&mejor_solucion);
                
                // Calcula la diferencia de costo entre la solucion actual y el vecino
                let costo_actual = solucion_actual.funcion_objetivo(self.delta);
                let costo_vecino = vecino.funcion_objetivo(self.delta);
                let delta = costo_actual - costo_vecino;

                // Determina la probabilidad de aceptacion
                let probabilidad_aceptacion = Self::probabilidad_aceptacion(delta, temperatura);
                
                total_iterations += 1;

                // Probabilistic acceptance
                if delta < 0.0 || rng.random::<f64>() < probabilidad_aceptacion {
                    solucion_actual = vecino;
                    total_accepted += 1;

                    // Actualiza la mejor solucion encontrada si es mejor que la actual
                    if vecino.funcion_objetivo(self.delta) < mejor_solucion.funcion_objetivo(self.delta) {
                        mejor_solucion = solucion_actual;
                        
                        // Esto es opcional debug output
                        if self.show_progress {
                            println!("Mejora encontrada:");
                            mejor_solucion.display();
                            println!("Temperatura actual recocido: {}", temperatura);
                            println!("Función objetivo: {}", mejor_solucion.funcion_objetivo(self.delta));
                            println!("--------------------------------");
                        }
                    }
                }
            }
            
            // Cool down the temperature del recocido
            temperatura *= self.tasa_enfriamiento;
        }

        // Estadisticas del recocido para debug
        if self.show_progress {
            println!("Estadísticas de exploración:");
            println!("Total de iteraciones: {}", total_iterations);
            println!("Soluciones aceptadas: {} ({:.2}%)", 
                total_accepted, 
                (total_accepted as f64 / total_iterations as f64) * 100.0 
            );
            println!("Mejor solución encontrada:");
            mejor_solucion.display();
        }

        mejor_solucion
    }
}