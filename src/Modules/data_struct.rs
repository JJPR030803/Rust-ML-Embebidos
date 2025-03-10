use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
pub struct Data {
    Timestamp: u64,
    Temperature: f64,
    Humidity: f64,
}

pub fn read_csv(filepath: &str) -> Result<Vec<Data>, Box<dyn Error>> {
    let mut reader = Reader::from_path(filepath)?;
    let mut data_vec: Vec<Data> = Vec::new();

    // Lee cada fila del csv y lo convierte a struct
    for line in reader.deserialize() {
        let row: Data = line?;
        data_vec.push(row); //Crea una lista de datos
    }

    Ok(data_vec)
}

// Funcion para extraer una columna para analisis o lo que sea
pub fn extract_series(data: &[Data], field: &str) -> Vec<f64> {
    match field {
        "Temperature" => data.iter().map(|d| d.Temperature).collect(),
        "Humidity" => data.iter().map(|d| d.Humidity).collect(),
        _ => panic!("Unsupported field: {}", field),
    }
}
