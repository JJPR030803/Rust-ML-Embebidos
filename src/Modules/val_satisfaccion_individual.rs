use super::satisfaccion_mult_categorias::HashedSatisfaccionCategorias;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct ValoresSatisfaccionIndividual {
    pub minimo: f64,
    pub maximo: f64,
    pub temp_actual: f64,
    pub temp_objetivo: f64,
    pub satisfaccion_actual:f64,
    pub satisfaccion_objetivo:f64,
    pub costo_unitario: f64,
    pub peso_satisfaccion: f64,
    pub checksum: f64,
}
/// Calcula el valor de la función objetivo para evaluar el balance entre costo y satisfacción.
///
/// La función objetivo utiliza el concepto de "Marginal Rate of Substitution" (MRS),
/// que es la relación entre el costo unitario y el peso de satisfacción, para determinar
/// el impacto relativo de los cambios en costo y satisfacción.
///
/// # Parámetros
/// - `delta`: `f64` - Un valor adicional que se suma al resultado de la función objetivo.
///            Este parámetro puede ser utilizado para ajustar el resultado según necesidades específicas.
///
/// # Retorno
/// - `f64`: El valor calculado de la función objetivo, que incluye un término de penalización
///          si el costo y la satisfacción tienen cambios en la misma dirección (positiva o negativa).
///
/// # Detalles
/// - `mrs`: Calcula la relación entre el costo unitario y el peso de satisfacción.
/// - `x`: Representa la diferencia en costo calculada por la función `diferencia_costo`.
/// - `y`: Representa la diferencia en satisfacción calculada por la función `diferencia_satisfaccion`.
/// - `objective_value`: Calcula el valor base de la función objetivo como `mrs * (x - y).abs() + delta`.
/// - `penalty`: Si tanto `x` como `y` son positivos o negativos (es decir, tienen la misma dirección),
///              se añade un término de penalización basado en `mrs * (x - y).abs()`.
///
/// # Ejemplo
/// ```rust
/// let valores = ValoresSatisfaccionIndividual::new(0.0, 100.0, 50.0, 75.0, 5.0, 10.0, 10.0, 5.0);
/// let delta = 2.0;
/// let resultado = valores.funcion_objetivo(delta);
/// println!("El valor de la función objetivo es: {}", resultado);
/// ```
///
/// # Nota
/// Esta función es crítica para evaluar el balance entre costo y satisfacción en el sistema.
/// No se recomienda modificarla sin un entendimiento profundo de su lógica.
impl ValoresSatisfaccionIndividual {
    ///#Args:
    /// -minimo valor minimo ejemplo: 0.0
    /// -maximo valor maximo ejemplo: 100.0
    /// -valor actual valor actual ejemplo: 50.0
    /// -valor objetivo valor objetivo ejemplo: 75.0
    /// -peso satisfaccion peso satisfaccion ejemplo: 5.0
    /// -costo costo unitario ejemplo: 10.0
    pub fn new(
        minimo: f64,
        maximo: f64,
        temp_actual: f64,
        temp_objetivo: f64,
        satisfaccion_actual:f64,
        satisfaccion_objetivo:f64,
        costo_unitario: f64,
        peso_satisfaccion: f64,
    ) -> Self {
        ValoresSatisfaccionIndividual {
            minimo,
            maximo,
            temp_actual,
            temp_objetivo,
            satisfaccion_actual,
            satisfaccion_objetivo,
            costo_unitario,
            peso_satisfaccion,
            checksum:0.0,
        }
    }
    //TODO actualilzar esto
    pub fn display(&self) {
        println!(
            "Minimo: {}, Maximo: {}",
            self.minimo, self.maximo
        );
        println!(
            "Valor Actual: {}, Valor Objetivo: {}",
            self.temp_actual, self.temp_objetivo
        );
        println!("Satisfaccion Actual: {}, Satisfaccion Objetivo: {}",self.satisfaccion_actual,self.satisfaccion_objetivo);
        println!(
            "Costo unitario: {} pesos por unidad, Peso satisfaccion: {}",
            self.costo_unitario, self.peso_satisfaccion
        );
        println!(
            "Temperatura: {}, Satisfaccion: {}\n",
        self.temp_actual,self.satisfaccion_actual);
        println!("Costo de cambiar la temperatura: {}, Costo de cambiar la satisfaccion: {}\n",self.diferencia_costo(),self.diferencia_satisfaccion());

    }
    pub fn to_hashed_satisfaccion(&self, categoria: String) -> HashedSatisfaccionCategorias {
        HashedSatisfaccionCategorias::convertir_a_hashmap(categoria, self.clone())
    }

    ///Regresa f64 valores de satisfaccion en este orden
    ///(minimo, maximo, valor_actual, valor_objetivo, peso_satisfaccion, costo_unitario)
    pub fn unwrap(&self) -> (f64, f64, f64, f64, f64, f64,f64,f64) {
        (
            self.minimo,
            self.maximo,
            self.temp_actual,
            self.temp_objetivo,
            self.satisfaccion_actual,
            self.satisfaccion_objetivo,
            self.costo_unitario,
            self.peso_satisfaccion,
        )
    }

    pub fn calculate_checksum(&mut self) -> f64 {
        // Original total cost components
        let original_cost = self.costo_unitario * self.temp_actual;
        let original_satisfaction = self.satisfaccion_actual * self.peso_satisfaccion;
        
        // Total initial "energy"
        let total = original_cost + original_satisfaction;
        
        self.checksum = total;
        total
    }
    pub fn checksum(&self) -> bool {
        // Calcula la "energia total" del sistema osea que no se hayan modificado valores que no deberian
        let current_cost = self.costo_unitario * self.temp_actual;
        let current_satisfaction = self.satisfaccion_actual * self.peso_satisfaccion;
        let current_total = current_cost + current_satisfaction;
        
        // Permite desviaciones pequeñas por redondeo
        let tolerance = self.checksum * 0.05;
        
        (current_total - self.checksum).abs() <= tolerance
    }
    
    ///Funcion que calcula el costo de cambiar el valor actual a un nuevo valor
    ///Regresa el costo de cambiar el valor actual a un nuevo valor
    ///#Parametros:
    ///- nuevo_valor: f64 - El nuevo valor al que se desea cambiar el valor actual
    pub fn diferencia_costo(&self)->f64{
        ((self.costo_unitario*self.temp_actual) - (self.costo_unitario*self.temp_objetivo)).abs()
    }
    ///Funcion que calcula el costo de satisfaccion de cambiar el valor actual a un nuevo valor
    ///Regresa el valor de satisfaccion de cambiar el valor actual a un nuevo valor
    ///#Parametros:
    ///- nuevo_valor: f64 - El nuevo valor al que se desea cambiar el valor actual
    pub fn diferencia_satisfaccion(&self) -> f64 {
        ((self.satisfaccion_actual*self.peso_satisfaccion) - (self.satisfaccion_objetivo*self.peso_satisfaccion)).abs()
    }

    //Funcion que calcula el valor de la funcion objetivo no modifiquen esto tarde mucho tiempo en que funcionara
    // mrs calcula el ratio de costo y satisfaccion
    //Delta es un valor para modificar la funcion objetivo se podria quitar pero no quiero arriesgarme
    //Aplica un penalty si el costo y la satisfaccion tienen cambios en la misma direccion

        pub fn funcion_objetivo(&self,delta:f64) -> f64 {
            let mrs = self.costo_unitario / self.peso_satisfaccion;
            let x = self.diferencia_costo();
            let y = self.diferencia_satisfaccion();
            let objective_value = mrs * (x -y).abs() + delta;
            let penalty = if (x > 0.0 && y > 0.0) || (x < 0.0 && y < 0.0) {
                mrs * (x - y).abs()
            } else {
                0.0
            };
            objective_value + penalty
        }
        
    
}

#[test]
fn test_fun_objetivo(){
    let mut valor = ValoresSatisfaccionIndividual {
        minimo: 0.0,
        maximo: 40.0,
        temp_actual: 20.0,
        temp_objetivo: 26.0,
        satisfaccion_actual: 5.0,
        satisfaccion_objetivo: 10.0,
        costo_unitario: 5.0,
        peso_satisfaccion: 5.0,
        checksum:0.0,
    };
    let total = valor.calculate_checksum();

    dbg!(total);
}
