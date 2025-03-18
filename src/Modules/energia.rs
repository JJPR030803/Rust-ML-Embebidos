use super::data_structs::Energia;

impl Energia {
    pub fn new(energia_actual: f64, peso: f64, costo_cambio: f64, is_min: bool) -> Energia {
        Energia {
            energia_actual,
            peso,
            costo_cambio,
            is_min,
        }
    }
    pub fn calc_satisfaccionenergia(self, valor_analizar: f64) -> f64 {
        if self.is_min {
            if valor_analizar > self.energia_actual {
                // When is_min is true and the analyzed value is greater than reference
                return self.costo_cambio * (valor_analizar - self.energia_actual);
            } else {
                // When is_min is true but analyzed value is not greater than reference
                return 0.0; // Assuming 0 cost when no change needed
            }
        } else {
            if valor_analizar < self.energia_actual {
                // When is_min is false and the analyzed value is less than reference
                return self.costo_cambio * (self.energia_actual - valor_analizar); // Note: I flipped the order to make it positive
            } else {
                // When is_min is false but analyzed value is not less than reference
                return 0.0; // Assuming 0 cost when no change needed
            }
        }
    }
}

#[test]
fn test_func() {
    let configuracion_actual = Energia::new(20.0, 0.5, 100.0, true);
    let configuracion_objetivo = Energia::new(30.0, 0.5, 100.0, false);

    let satisfaccion =
        configuracion_actual.calc_satisfaccionenergia(configuracion_objetivo.energia_actual);

    println!("Satisfaccion: {}", satisfaccion);
}
