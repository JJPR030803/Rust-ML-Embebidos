#[derive(Clone, Copy)]
pub struct ValoresSensores {
    humedad: ValoresSatisfaccion,
    temperatura: ValoresSatisfaccion,
    luminosidad: ValoresSatisfaccion,
}

#[derive(Clone, Copy)]
pub struct ValoresSatisfaccion {
    pub valor: f64,
    pub minimo: f64,
    pub maximo: f64,
    pub is_min: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Energia {
    pub energia_actual: f64,
    pub peso: f64,
    pub costo_cambio: f64,
    pub is_min: bool,
}
