#![allow(dead_code)]
#![allow(unused_variables)]

use rulinalg::matrix::{Matrix, BaseMatrix};

pub trait ActivationFunction {
    fn apply_activation_func_to_matrix(matrix: Matrix<f64>) -> Matrix<f64> where Self: Sized;
    fn apply_derivative_of_activation_func_to_matrix(matrix: Matrix<f64>) -> Matrix<f64> where Self: Sized;
    fn get_name() -> String where Self: Sized;
}

// pub struct ActivationFunctionFactory {
//     activation_func_map: HashMap<String, Box<dyn ActivationFunction>>
// }

// impl ActivationFunctionFactory {
//     pub fn new() -> ActivationFunctionFactory {
//         let sigmoid = Box::new(SigmoidActivationFunction {});
//         let funcs = HashMap::new();
//         funcs.insert(sigmoid.get_name(), sigmoid);

//         let factory = ActivationFunctionFactory{
//             activation_func_map: funcs
//         };

//         factory
//     }
// }

pub struct SigmoidActivationFunction {}

impl ActivationFunction for SigmoidActivationFunction {
    fn apply_activation_func_to_matrix( matrix: Matrix<f64>) -> Matrix<f64> {
        let mut data = Vec::new();

        for i in 0..matrix.rows() {
            let val = matrix.data()[i]; //only one col so this is fine
            let res = 1f64 / (1f64 + (-val).exp());

            data.push(res);
        }

        Matrix::new(matrix.rows(), matrix.cols(), data)
    }

    fn apply_derivative_of_activation_func_to_matrix(matrix: Matrix<f64>) -> Matrix<f64> {
        let mut data = Vec::new();

        for i in 0..matrix.rows() {
            let val = matrix.data()[i]; //only one col so this is fine
            let res = 1f64 / (1f64 - val);

            data.push(res);
        }

        Matrix::new(matrix.rows(), matrix.cols(), data)
    }

    fn get_name() -> String {
        String::from("SIGMOID")
    }
}