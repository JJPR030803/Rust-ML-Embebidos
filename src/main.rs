mod Modules;
use Modules::iqr::IQRQuartiles;
use Modules::matrix::Matrix;
use Modules::normalizacion::normalizacion;
use csv::Reader;
use rand::Rng;
use rand::seq::IteratorRandom;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let column_data = read_csv()?;
    let decimal_data: Vec<f32> = column_data.iter().map(|x| x.parse().unwrap()).collect();
    let (max_value, min_value, eo, normalized) = calculate_temperature_satisfaction(&column_data);
    let cuartiles = IQRQuartiles::calc_cuartiles(&decimal_data);

    let mut matrix = Matrix::new(100, 30);
    let mut cuartiles_matrix = Matrix::new(100, 30);
    let mut cuartiles_data: Vec<Vec<f32>> = cuartiles_matrix.data.clone();

    matrix.fill_random(0.0, 40.0, &mut rng);

    for i in 0..cuartiles_data.len() {
        let tupla_cuartil = IQRQuartiles::calc_cuartiles(&matrix.data[i]).unwrap();
        cuartiles_data[i] = vec![
            tupla_cuartil.0,
            tupla_cuartil.1,
            tupla_cuartil.2,
            tupla_cuartil.3,
        ];
        println!("Indice: {}, Cuartiles: {:?}", i, cuartiles_data[i]);
    }

    Ok(())
}

fn read_csv() -> Result<Vec<String>, Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(
        "/home/batman/Documents/UAT/Octavo/Diseño_El_Embebidos/Rust/proyecto_limpio/embebidos_limpo/sensor_data.csv",
    )?;
    let mut rdr = Reader::from_reader(file);

    // Get the column name and its index
    let column_name = "Temperature"; // Replace with your column name
    let headers = rdr.headers()?.clone();
    let column_index = headers
        .iter()
        .position(|h| h == column_name)
        .ok_or_else(|| format!("Column '{}' not found", column_name))?;

    // Extract the column data
    let mut column_data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if let Some(value) = record.get(column_index) {
            column_data.push(value.to_string());
        }
    }

    Ok(column_data)
}

fn calculate_temperature_satisfaction(column_data: &[String]) -> (f32, f32, f32, f32) {
    //Calcular satisfaccion
    let max_value: f32 = column_data.iter().max().unwrap().parse().unwrap();
    let min_value: f32 = column_data.iter().min().unwrap().parse().unwrap();
    let eo: f32 = column_data
        .iter()
        .choose(&mut rand::rng())
        .unwrap()
        .parse()
        .unwrap();

    let normalizado = normalizacion(eo, min_value, max_value);

    (max_value, min_value, eo, normalizado)
}
