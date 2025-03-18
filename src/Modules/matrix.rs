use std::arch::x86_64::_SIDD_CMP_RANGES;

use rand::Rng;
use rand::distr::{Distribution, Uniform};
use statrs::distribution;
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(num_rows: usize, num_columns: usize) -> Self {
        Matrix {
            rows: num_rows,
            cols: num_columns,
            data: vec![vec![0.0; num_columns]; num_rows],
        }
    }
    pub fn display(&self) {
        for row in &self.data {
            println!("Row:");
            for value in row {
                print!("{:.2},", value);
            }
            println!();
        }
    }

    pub fn fill_random<R: Rng>(&mut self, min: f32, max: f32, rng: &mut R) {
        if min > max {
            panic!("Minimum value cannot be greater than maximum value");
        }
        let distribution = Uniform::new(min, max);

        for row in &mut self.data {
            for value in row {
                *value = rng.random_range(min..max);
            }
        }
    }
}
