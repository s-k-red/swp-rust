use itertools::Itertools;
use serde::{Serialize, Deserialize};

use super::bot::Bot;

#[derive(Serialize, Deserialize)]
pub struct SerializableBot {
    pub id: String,
    pub last_deaths: i8,
    pub last_amount_rounds: usize,
    pub input_nodes: usize,
    pub hidden_layers: usize,
    pub hidden_nodes: usize,
    pub output_nodes: usize,
    pub activation_function_key: String,
    pub weights: Vec<Vec<f64>>,
    pub biases: Vec<Vec<f64>>,
}

impl From<Bot> for SerializableBot {
    fn from(bot: Bot) -> Self {
        SerializableBot { 
            id: bot.id, 
            last_amount_rounds: bot.round_index,
            last_deaths: bot.last_deaths,
            input_nodes: bot.brain.input_nodes, 
            hidden_layers: bot.brain.hidden_layers, 
            hidden_nodes: bot.brain.hidden_nodes, 
            output_nodes: bot.brain.output_nodes, 
            activation_function_key: bot.brain.activation_function_key, 
            weights: bot.brain.weights.iter()
                        .map(|w| w.data().clone()).collect_vec(), 
            biases: bot.brain.biases.iter()
                        .map(|w| w.data().clone()).collect_vec() 
        }
    }
}