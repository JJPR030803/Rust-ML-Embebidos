use super::data_structs::{ValoresSatisfaccion, ValoresSensores};
use std::collections::HashMap;

pub struct satisfaccion {
    vector_preferencias: HashMap<String, ValoresSatisfaccion>,
    vector_actual: HashMap<String, ValoresSatisfaccion>,
    peso: f64,
    is_min: bool,
}

impl satisfaccion {
    pub fn new(
        vector_preferencias: HashMap<String, ValoresSatisfaccion>,
        vector_actual: HashMap<String, ValoresSatisfaccion>,
        peso: f64,
        is_min: bool,
    ) -> satisfaccion {
        satisfaccion {
            vector_preferencias,
            vector_actual,
            peso,
            is_min,
        }
    }

    pub fn calcular_satisfaccion(&self) -> HashMap<String, f64> {
        let mut resultados = HashMap::new();
        for (k, v) in self.vector_actual.clone().iter() {
            let valores_preferencias = self.vector_preferencias.get(k).unwrap().clone();
            let satisfaccion = if self.is_min {
                Self::calcular_min(v.valor, v.minimo, v.maximo)
            } else {
                Self::calcular_max(v.valor, v.minimo, v.maximo)
            };
            resultados.insert(k.clone(), satisfaccion * self.peso);
        }
        resultados
    }

    pub fn calcular_min(valor_analizar: f64, vmin: f64, vmax: f64) -> f64 {
        (vmax - valor_analizar) / (vmax - vmin)
    }

    fn calcular_max(valor_analizar: f64, vmin: f64, vmax: f64) -> f64 {
        (valor_analizar - vmin) / (vmax - vmin)
    }
}
#[test]
fn test_calc_satisfaccion() {
    let vector_preferencias = HashMap::from([
        (
            "item1".to_string(),
            ValoresSatisfaccion {
                valor: 1.0,
                minimo: 0.0,
                maximo: 10.0,
                is_min: true,
            },
        ),
        (
            "item2".to_string(),
            ValoresSatisfaccion {
                valor: 2.0,
                minimo: 0.0,
                maximo: 10.0,
                is_min: false,
            },
        ),
    ]);
    let vector_actual = HashMap::from([
        (
            "item1".to_string(),
            ValoresSatisfaccion {
                valor: 2.0,
                minimo: 0.0,
                maximo: 10.0,
                is_min: true,
            },
        ),
        (
            "item2".to_string(),
            ValoresSatisfaccion {
                valor: 5.0,
                minimo: 0.0,
                maximo: 10.0,
                is_min: false,
            },
        ),
    ]);
    let satisfaccion = satisfaccion::new(vector_preferencias, vector_actual, 1.0, true);
    let resultados = satisfaccion.calcular_satisfaccion();
    dbg!("{:?}", resultados);
}
