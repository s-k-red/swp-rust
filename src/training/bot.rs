#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{neural_net::NeuralNet, config::{OUTPUT_NODES, INPUT_NODES, HIDDEN_LAYERS}, game_states::GameState, components::{GameStore, Robot, MAX_HP, Board, Player}, datatypes::Position};
use itertools::Itertools;
use uuid::Uuid;

#[derive(Clone)]
pub struct Bot{
    brain: NeuralNet,
    pub id: String,
    pub normalized_fitness: f64,
    pub own_fitness: f64
}

impl Bot {
    pub fn new_random() -> Bot {
        let id = Uuid::new_v4().to_string();
        Bot { 
            brain: NeuralNet::new(INPUT_NODES, HIDDEN_LAYERS, INPUT_NODES, OUTPUT_NODES), 
            id, 
            normalized_fitness: 0.0, 
            own_fitness: 0.0
        }
    }

    pub fn new(brain: NeuralNet) -> Bot {
        let id = Uuid::new_v4().to_string();
        Bot {
            id: id.clone(),
            brain, 
            normalized_fitness: 0.0, 
            own_fitness: 0.0
        }
    }

    pub fn calc_own_fitness(&mut self, game_store: &GameStore) -> f64 {
        //maybe this has to be done after each round in the future
        let mut fitness = 0.0;
        let mut robot = game_store.robots.iter().find_or_first(|r| r.user_name.eq(&self.id));
        let fake_robot = Robot::new(String::from("lul"), Position {x:0, y:0});

        if robot.is_none(){
            return 0.0;
        }

        if game_store.winners.contains(&self.id) {
            fitness += 1.0;
        } 
        
        fitness += robot.get_or_insert(&fake_robot).greatest_checkpoint_reached as f64 / 6.0; //TODO!!!!! change to max num of checkpoints

        fitness -= (robot.get_or_insert(&fake_robot).deaths as f64 / 2.0).exp(); //2 deaths is bad but oookay but from there on its really bad

        fitness -= robot.get_or_insert(&fake_robot).hp as f64 / MAX_HP as f64;

        self.own_fitness = fitness.clone();

        fitness
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