#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{neural_net::NeuralNet, config::{OUTPUT_NODES, INPUT_NODES, HIDDEN_LAYERS}};
use uuid::Uuid;

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
}

impl Eq for Bot {}

impl PartialEq for Bot {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}