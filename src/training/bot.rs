#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{neural_net::NeuralNet, config::{OUTPUT_NODES, INPUT_NODES, HIDDEN_LAYERS}, game_states::GameState, components::GameStore};
use uuid::Uuid;

#[derive(Clone)]
pub struct Bot{
    brain: NeuralNet,
    pub id: String
}

impl Bot {
    pub fn new_random() -> Bot {
        Bot { brain: NeuralNet::new(INPUT_NODES, HIDDEN_LAYERS, INPUT_NODES, OUTPUT_NODES), id:  Uuid::new_v4().to_string() }
    }

    pub fn new(brain: NeuralNet) -> Bot {
        Bot {id:  Uuid::new_v4().to_string(), brain}
    }

    pub fn calc_fitness(&self, store: GameStore) -> f64 {
        todo!()
    }

    pub fn mutate(&mut self){
        self.brain.mutate();
    }

    pub fn save_brain(&self){
        self.brain.save();
    }
}

impl Eq for Bot {}

impl PartialEq for Bot {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}