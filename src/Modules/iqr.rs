///# Implementación de la estructura IQRQuartiles

pub struct IQRQuartiles {
    pub q1: f32,
    pub q2: f32,
    pub q3: f32,
    pub max: f32,
}

///# Implementación de los métodos de la estructura IQRQuartiles
impl IQRQuartiles {
    pub fn new(q1: f32, q2: f32, q3: f32, max: f32) -> Self {
        IQRQuartiles { q1, q2, q3, max }
    }
    //metodo para calcular los cuartiles ordena la lista de datos de menor a mayor y usa los indices para calcular los cuartiles
    pub fn calc_cuartiles(data: &Vec<f32>) -> Self {
        let mut sorted_data = data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let q1 = sorted_data[0];
        let max = sorted_data[sorted_data.len() - 1];
        let q2 = sorted_data[sorted_data.len() / 2];
        let q1 = sorted_data[sorted_data.len() / 4];
        let q3 = sorted_data[sorted_data.len() * 3 / 4];

        IQRQuartiles::new(q1, q2, q3, max)
    }
    //Metodo para convertir el objeto IQRQuartiles a una tupla de f32 (mas facil de manipular)
    pub fn unwrap(&self) -> (f32, f32, f32, f32) {
        (self.q1, self.q2, self.q3, self.max)
    }
    //Display pues displayea sepan leer
    pub fn display(&self) {
        println!(" q1: {}", self.q1);
        println!(" q2: {}", self.q2);
        println!(" q3: {}", self.q3);
        println!("max o q4: {}", self.max);
    }
}
