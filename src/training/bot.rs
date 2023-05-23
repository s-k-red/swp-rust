#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    components::{Board, GameStore, Player, Robot, MAX_HP},
    config::{HIDDEN_LAYERS, INPUT_NODES, OUTPUT_NODES},
    datatypes::Position,
    game_states::GameState,
    neural_net::NeuralNet,
};
use itertools::Itertools;
use uuid::Uuid;

#[derive(Clone)]
pub struct Bot {
    brain: NeuralNet,
    pub id: String,
    pub normalized_fitness: f64,
    pub own_fitness: f64,
}

impl Bot {
    pub fn new_random() -> Bot {
        let id = Uuid::new_v4().to_string();
        Bot {
            brain: NeuralNet::new(INPUT_NODES, HIDDEN_LAYERS, INPUT_NODES, OUTPUT_NODES),
            id,
            normalized_fitness: 0.0,
            own_fitness: 0.0,
        }
    }

    pub fn new(brain: NeuralNet) -> Bot {
        let id = Uuid::new_v4().to_string();
        Bot {
            id: id.clone(),
            brain,
            normalized_fitness: 0.0,
            own_fitness: 0.0,
        }
    }

    pub fn calc_own_fitness(&mut self, game_store: &GameStore) -> f64 {
        //maybe this has to be done after each round in the future
        let mut fitness = 0.0;
        let robot = game_store
            .robots
            .iter()
            .find_or_first(|r| r.user_name.eq(&self.id))
            .unwrap();

        // match &game_store.winners {
        //     Some(winners) => {
        //         if winners.contains(&robot.user_name) {
        //             fitness += 1.0;
        //         }
        //     }
        //     None => todo!(),
        // } //TODODODODOD

        fitness += robot.greatest_checkpoint_reached as f64 / 6.0; //TODO!!!!! change to max num of checkpoints

        fitness -= (robot.deaths as f64 / 2.0).exp(); //2 deaths is bad but oookay but from there on its really bad

        fitness -= robot.hp as f64 / MAX_HP as f64;

        self.own_fitness = fitness.clone();

        return fitness;
    }

    pub fn mutate(&mut self) {
        self.brain.mutate();
    }

    pub fn save_brain(&self) {
        self.brain.save();
    }

    pub fn play_card(&self, input: Vec<f64>) -> usize{
        self.brain.guess(input).iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index).unwrap()
    }
}

impl Eq for Bot {}

impl PartialEq for Bot {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
