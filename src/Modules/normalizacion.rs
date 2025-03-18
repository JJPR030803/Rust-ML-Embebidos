/// #Que hace:
/// emin lo menos quepuedo gastar
/// emax lo maximo
/// eo objetivo a llegar
/// calcula el costo a llegar a eo
pub fn normalizacion(eo: f32, emin: f32, emax: f32) -> f32 {
    (eo - emin) / (emax - emin)
}

#[test]
fn test_normalizacion() {
    let eo = 453.0;
    let emin = 0.0;
    let emax = 603.0;

    let normalized = normalizacion(eo, emin, emax);

    dbg!(normalized);
    assert!(normalized == 0.75124377);
}
