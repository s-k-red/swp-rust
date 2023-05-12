#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;

use rand::Rng;
use rulinalg::matrix::{Matrix, BaseMatrix};

pub fn random_matrix(rows: usize, cols:usize, from:f64, to:f64) -> Matrix<f64> {
    let mut vals = Vec::new();
    let mut rng = rand::thread_rng();

    for row in 0..rows {
        for col in 0..cols {
            vals.push(rng.gen_range(-1.0..1.0))
        }
    }

    Matrix::new(rows, cols, vals)
}

pub fn save_matrix(matrix: Matrix<f64>, filepath: &str){
    let vals = matrix.data();
    let mut output = String::new();

    for row in 0..matrix.rows(){
        for col in 0..matrix.cols() {
            let val = vals.get(col * row+1);
            output.push_str(&val.unwrap().to_string());
            output.push('|');
        }
        output.push('\n');
    }

    let res = fs::write(filepath, output);
}

pub fn copy_matrix_vector(from: Vec<Matrix<f64>>) -> Vec<Matrix<f64>>{
    let vals = Vec::new();

    vals
}

pub fn array_to_matrix(i: Vec<f64>) -> Matrix<f64> {
    Matrix::new(i.len(), 1, i)
}