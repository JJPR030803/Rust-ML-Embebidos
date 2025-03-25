mod Modules;
use Modules::*;
use csv::Reader;
use std::error::Error;
use std::fs::File;
use val_satisfaccion_individual::ValoresSatisfaccionIndividual;

fn main() -> Result<(), Box<dyn Error>> {

    // Define variables for easy modification
    let temp_min = 10.0;
    let temp_max = 50.0;
    let temp_actual = 20.0;
    let temp_objetivo = 25.0;
    let temp_costo_unitario = 1.0;
    let temp_peso_satisfaccion = 10.0;
    let sat_actual = 10.0;
    let sat_objetivo = 100.0;

    let humedad_min = 10.0;
    let humedad_max = 90.0;
    let humedad_actual = 80.0;
    let humedad_objetivo = 50.0;
    let hum_sat_actual = 50.0;
    let hum_sat_objetivo = 100.0;
    let humedad_costo_unitario = 50.0;
    let humedad_peso_satisfaccion = 5.0;

    // Use variables to create ValoresSatisfaccionIndividual instances
    let mut satisfaccion_temp = ValoresSatisfaccionIndividual::new(
        temp_min,
        temp_max,
        temp_actual,
        temp_objetivo,
        sat_actual,
        sat_objetivo,
        temp_costo_unitario,
        temp_peso_satisfaccion,
    );
    let sum1 = satisfaccion_temp.calculate_checksum();
    let satisfaccion_humedad = ValoresSatisfaccionIndividual::new(
        humedad_min,
        humedad_max,
        humedad_actual,
        humedad_objetivo,
        hum_sat_actual,
        hum_sat_objetivo,
        humedad_costo_unitario,
        humedad_peso_satisfaccion,
    );

    let recocido_temp = annealing2::Annealing::new(1000.0, 0.995, 0.01, 1000, satisfaccion_temp, true,10.0);
    let mut resultado = recocido_temp.run();
    let sum2 = resultado.calculate_checksum();

    
    


    resultado.display();
    if resultado.checksum(){
        println!("Checksum correcto");
        println!("Checksum 1: {}",sum1);
        println!("Checksum 2: {}",sum2);
    }else{
        println!("Checksum incorrecto");
    }
    resultado.display();

    Ok(())
}

fn read_csv() -> Result<Vec<String>, Box<dyn Error>> {
    // Open the CSV file
    let file = File::open(
        "/home/batman/Documents/UAT/Octavo/Diseño_El_Embebidos/Rust/proyecto_limpio/embebidos_limpo/sensor_data.csv",
    )?;
    let mut rdr = Reader::from_reader(file);

    // Obtiene la columna del csv solo la columna
    let column_name = "Temperature";
    let headers = rdr.headers()?.clone();
    let column_index = headers
        .iter()
        .position(|h| h == column_name)
        .ok_or_else(|| format!("Column '{}' not found", column_name))?; //Itera por cada columna hasta encontrar el que se llame igual en este caso Temperature

    // Extrae los datos no se muy bien como funciona este ya luego
    let mut column_data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if let Some(value) = record.get(column_index) {
            column_data.push(value.to_string());
        }
    }

    Ok(column_data)
}
