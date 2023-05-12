#![allow(dead_code)]
#![allow(unused_variables)]

use rulinalg::matrix::Matrix;

use crate::neural_net::activation_function::SigmoidActivationFunction;
mod matrix_utils;
mod activation_function;


#[derive(Debug, Clone)]
pub struct NeuralNet{
    input_nodes: usize,
    hidden_layers: usize,
    hidden_nodes: usize,
    output_nodes: usize,
    learning_rate: f64,
    activation_function_key: String,
    weights: Vec<Matrix<f64>>,
    biases: Vec<Matrix<f64>>
}


impl NeuralNet {
    pub fn new(input_nodes:usize, hidden_layers:usize, hidden_nodes:usize, output_nodes:usize) -> NeuralNet {
        NeuralNet {
            input_nodes,
            hidden_layers,
            hidden_nodes,
            output_nodes,
            learning_rate: 0.1f64,
            activation_function_key: String::from("SIGMOID"),
            weights: NeuralNet::init_weights(input_nodes, hidden_nodes, output_nodes, hidden_layers),
            biases: NeuralNet::init_biases(hidden_nodes, output_nodes, hidden_layers)
        }
    }

    fn init_weights(input_nodes:usize, hidden_nodes:usize, output_nodes:usize, hidden_layers:usize) -> Vec<Matrix<f64>> {
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
    
    fn init_biases(hidden_nodes:usize, output_nodes:usize, hidden_layers:usize) -> Vec<Matrix<f64>> {
        let mut biases = Vec::new();
    
        for num in 0..hidden_layers+1 {
            if num == hidden_layers {
                biases.push(matrix_utils::random_matrix(output_nodes, 1, -1.0, 1.0))
            } else {
                biases.push(matrix_utils::random_matrix(hidden_nodes, 1, -1.0, 1.0))
            }
        }
    
        biases
    }
}

impl NeuralNet {
    pub fn guess(&self, input: Vec<f64>) -> Vec<f64> {
        assert!(input.len() == self.input_nodes, "Wrong dimensions!");

        let activation_function = SigmoidActivationFunction{};

        
    }
}


