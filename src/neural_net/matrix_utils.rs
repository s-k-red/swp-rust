#![allow(dead_code)]
#![allow(unused_variables)]

use rand::Rng;
use rulinalg::matrix::Matrix;

pub fn random_matrix(rows: usize, cols:usize, from:f64, to:f64) -> Matrix<f64> {
    let vals = Vec::new();



    Matrix::new(rows, cols, vals)
}

pub fn copy_matrix_vector(from: Vec<Matrix<f64>>) -> Vec<Matrix<f64>>{
    let vals = Vec::new();

    vals
}