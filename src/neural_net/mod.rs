#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Mul;

use itertools::Itertools;
use rulinalg::matrix::{Matrix, BaseMatrix};

use crate::{neural_net::activation_function::SigmoidActivationFunction, config::MUTATION_RATE};

use self::activation_function::ActivationFunction;
pub mod matrix_utils;
mod activation_function;


#[derive(Debug, Clone, PartialEq)]
pub struct NeuralNet{
    pub input_nodes: usize,
    pub hidden_layers: usize,
    pub hidden_nodes: usize,
    pub output_nodes: usize,
    pub learning_rate: f32,
    pub activation_function_key: String,
    pub weights: Vec<Matrix<f32>>,
    pub biases: Vec<Matrix<f32>>
}

impl Eq for NeuralNet {
    fn assert_receiver_is_total_eq(&self) {}
}


impl NeuralNet {
    pub fn new(input_nodes:usize, hidden_layers:usize, hidden_nodes:usize, output_nodes:usize) -> NeuralNet {
        NeuralNet {
            input_nodes,
            hidden_layers,
            hidden_nodes,
            output_nodes,
            learning_rate: 0.1f32,
            activation_function_key: String::from("SIGMOID"),
            weights: NeuralNet::init_weights(input_nodes, hidden_nodes, output_nodes, hidden_layers),
            biases: NeuralNet::init_biases(hidden_nodes, output_nodes, hidden_layers)
        }
    }

    fn init_weights(input_nodes:usize, hidden_nodes:usize, output_nodes:usize, hidden_layers:usize) -> Vec<Matrix<f32>> {
        let mut weights = Vec::new();
    
        for num in 0..hidden_layers+1 {
            if num == 0 {
                weights.push(matrix_utils::random_matrix(hidden_nodes, input_nodes, -1.0, 1.0));
            } else if num == hidden_layers {
                weights.push(matrix_utils::random_matrix(output_nodes, hidden_nodes, -1.0, 1.0));
            } else {
                weights.push(matrix_utils::random_matrix(hidden_nodes, hidden_nodes, -1.0, 1.0));
            }
        }
    
        weights
    }
    
    fn init_biases(hidden_nodes:usize, output_nodes:usize, hidden_layers:usize) -> Vec<Matrix<f32>> {
        let mut biases = Vec::new();
    
        for num in 0..hidden_layers+1 {
            if num == hidden_layers {
                biases.push(matrix_utils::random_matrix(output_nodes, 1, -1.0, 1.0));
            } else {
                biases.push(matrix_utils::random_matrix(hidden_nodes, 1, -1.0, 1.0));
            }
        }
    
        biases
    }
}

impl NeuralNet {
    pub fn guess(&self, input: Vec<f32>) -> Vec<f32> {
        assert!(input.len() == self.input_nodes, "Wrong dimensions!");
        
        let fun: SigmoidActivationFunction = SigmoidActivationFunction{};
        let mut output = Matrix::new(input.len(), 1, input);

        for i in 0..self.hidden_layers+1 {
            output = NeuralNet::calculate_layer(&self.weights[i], &self.biases[i], &output, &fun)
        }

        output.col(0).iter().copied().collect_vec()
    }

    pub fn mutate(&mut self){
        let mut weights = Vec::new();
        let mut biases = Vec::new();

        for w in self.weights.as_slice() {
            weights.push(matrix_utils::mutate(w, MUTATION_RATE));
        }

        for b in self.biases.as_slice() {
            biases.push(matrix_utils::mutate(b, MUTATION_RATE));
        }

        self.weights = weights;
        self.biases = biases;
    }

    pub fn save(&self){
        for (i, w) in self.weights.iter().enumerate() {
            let name = format!("{}weights.txt", i);
            matrix_utils::save_matrix(w.clone(), &name);
        }

        for (i, b)  in self.biases.iter().enumerate() {
            let name = format!("{}mut.txt", i);
            matrix_utils::save_matrix(b.clone(), &name);
        }
    }

    fn calculate_layer(weights: &Matrix<f32>, biases: &Matrix<f32>, input: &Matrix<f32>, func: &dyn ActivationFunction) -> Matrix<f32> {
        let result = weights.mul(input) + biases;

        NeuralNet::apply_activation_function(result, false, func)
    }

    fn apply_activation_function(input: Matrix<f32>, derivative: bool, func: &dyn ActivationFunction) -> Matrix<f32> {
        if derivative {
            func.apply_derivative_of_activation_func_to_matrix(input)
        } else {
            func.apply_activation_func_to_matrix(input)
        }
    }
}