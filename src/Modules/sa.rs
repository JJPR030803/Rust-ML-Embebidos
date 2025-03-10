use rand::Rng;
fn func_objetivo(x: f64) -> f64 {
    x.powi(2) + 4.0 * (5.0 * x).sin() //Funcion a minimizar
}

pub fn recocido_simulado(
    temp_inicial: f64,
    tasa_enfriamiento: f64,
    iteraciones: usize,
) -> (f64, f64) {
    let mut rng = rand::rng();

    //Solucion inicial aleatoria entre -5 y 5
    let mut x_actual = rng.random_range(-5.0..5.0);
    let mut temperatura = temp_inicial;

    for _ in 0..iteraciones {
        //Generar vecino aleatorio
        let x_nuevo = x_actual + rng.random_range(-1.0..1.0);

        let delta_e = func_objetivo(x_nuevo) - func_objetivo(x_actual);

        //Aceptar el nuuevo estado si es mejor  o con cierta probabilidad si es mejor
        if delta_e < 0.0 || rng.random::<f64>() < (delta_e / temperatura).exp() {
            x_actual = x_nuevo
        }

        temperatura *= tasa_enfriamiento;
    }

    (x_actual, func_objetivo(x_actual))
}
