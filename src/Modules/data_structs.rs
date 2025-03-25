#[derive(Debug, Clone, Copy)]
pub struct Energia {
    pub energia_actual: f64,
    pub peso: f64,
    pub costo_cambio: f64,
    pub is_min: bool,
}
