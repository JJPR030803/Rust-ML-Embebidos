use super::val_satisfaccion_individual::ValoresSatisfaccionIndividual;
use std::collections::HashMap;

pub struct HashedSatisfaccionCategorias {
    categoria: String,
    vector_preferencias: ValoresSatisfaccionIndividual,
}
//
impl HashedSatisfaccionCategorias {
    pub fn convertir_a_hashmap(
        categoria: String,
        vector_preferencias: ValoresSatisfaccionIndividual,
    ) -> Self {
        HashedSatisfaccionCategorias {
            categoria,
            vector_preferencias,
        }
    }

    pub fn get_categoria(&self) -> &String {
        &self.categoria
    }

    pub fn get_vector_preferencias(&self) -> &ValoresSatisfaccionIndividual {
        &self.vector_preferencias
    }

    pub fn display(&self) {
        println!("Columna: {}", self.categoria);
        println!("Datos: {:?}", self.vector_preferencias);
    }
    /*  Ya no funciona
        pub fn weighted_normalize(&self) -> f64 {
            let (min, max, v_actual, peso, v_objetivo) = self.vector_preferencias.unwrap();
            (v_actual - v_objetivo).abs() / (max - min) * peso
        }
    */
    pub fn calcular_min(valor_analizar: f64, vmin: f64, vmax: f64) -> f64 {
        (vmax - valor_analizar) / (vmax - vmin)
    }

    fn calcular_max(valor_analizar: f64, vmin: f64, vmax: f64) -> f64 {
        (valor_analizar - vmin) / (vmax - vmin)
    }
}
