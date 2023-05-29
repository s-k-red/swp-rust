#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;

use rand::Rng;
use rulinalg::matrix::{Matrix, BaseMatrix};

pub fn random_matrix(rows: usize, cols:usize, from:f32, to:f32) -> Matrix<f32> {
    let mut vals = Vec::new();
    let mut rng = rand::thread_rng();

    for row in 0..rows {
        for col in 0..cols {
            vals.push(rng.gen_range(-1.0..1.0))
        }
    }

    Matrix::new(rows, cols, vals)
}

pub fn save_matrix(matrix: Matrix<f32>, filepath: &str){
    let vals = matrix.data();
    let mut output = String::new();

    for row in 0..matrix.rows(){
        for col in 0..matrix.cols() {
            let val = vals.get(matrix.cols() * row + col);
            output.push_str(&val.unwrap().to_string());
            output.push('|');
        }
        output.push('\n');
    }

    let res = fs::write(filepath, output);
}

pub fn mutate(mat: &Matrix<f32>, rate: f32) -> Matrix<f32> {
    let mut vals = Vec::new();

    for row in 0..mat.rows() {
        let mut rng = rand::thread_rng();

        for col in 0..mat.cols() {
            let current_val = *mat.data().get(mat.cols() * row + col).unwrap();
            if rng.gen::<f32>() < rate { //TODO: maybe change?
                vals.push(rng.gen::<f32>() * 2.0 - 1.0); //TODO: maybe change? for now completely random
            } else {
                vals.push(current_val);   
            }
        }
    }

    Matrix::new(mat.rows(), mat.cols(), vals)
}