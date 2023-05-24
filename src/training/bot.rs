#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    components::{GameStore, MAX_HP, Card},
    config::{HIDDEN_LAYERS, INPUT_NODES, OUTPUT_NODES},
    neural_net::NeuralNet, datatypes::Position, serialization::{TileEntitySerialize, TileSerialize},
};
use itertools::Itertools;
use uuid::Uuid;

use super::input_builder;

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
            id,
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

        self.own_fitness = fitness;

        fitness
    }

    pub fn mutate(&mut self) {
        self.brain.mutate();
    }

    pub fn save_brain(&self) {
        self.brain.save();
    }

    pub fn play_cards(&self, gs: &GameStore, map: &Vec<TileSerialize>, checkpoints: &Vec<(usize, Position)>) -> Vec<Card> {
        let me = gs.players.iter().find(|p| p.user_name.eq(&self.id)).unwrap();
        let mut cards = me.cards_in_hand.clone();
        let legal_amount = std::cmp::min(5, cards.len());
        let mut played_cards: Vec<Card> = Vec::new();
        
        let mut init_res = self.brain.guess(input_builder::get_inputs(self, gs, &played_cards, map, checkpoints));
        while played_cards.is_empty() {
            let index = init_res.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index).unwrap();

            if index >= cards.len() || !cards[index].is_movement {
                init_res[index] = 0.0;
                continue;
            }

            played_cards.push(cards[index].clone());
            cards.remove(index);
        }
        
        for i in 1..legal_amount {
            let mut res = self.brain.guess(input_builder::get_inputs(self, gs, &played_cards, map, checkpoints));
            while played_cards.len() == i {
                
                let index = res.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(index, _)| index).unwrap();

                if index >= cards.len() {
                    res[index] = 0.0;
                    continue;
                }
                played_cards.push(cards[index].clone());
                cards.remove(index);
            }
        }

        played_cards
    }

    fn play_card(&self, input: Vec<f64>) -> usize{
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
