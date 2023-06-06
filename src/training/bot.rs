#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    components::{GameStore, MAX_HP, Card},
    config::{HIDDEN_LAYERS, INPUT_NODES, OUTPUT_NODES, CHECKPOINTS},
    neural_net::NeuralNet, datatypes::Position, serialization::{TileEntitySerialize, TileSerialize},
};
use itertools::Itertools;
use serde::Serialize;
use uuid::Uuid;

use super::input_builder;

#[derive(Clone)]
pub struct Bot {
    pub brain: NeuralNet,
    pub id: String,
    pub normalized_fitness: f32,
    pub own_fitness: f32,
    pub round_index: usize,
    pub last_deaths: i8,
    pub won: bool
}

impl Bot {
    pub fn new_random() -> Bot {
        let id = Uuid::new_v4().to_string();
        Bot {
            brain: NeuralNet::new(INPUT_NODES, HIDDEN_LAYERS, INPUT_NODES, OUTPUT_NODES),
            id,
            normalized_fitness: 0.0,
            own_fitness: 0.0,
            round_index: 0,
            last_deaths: 0,
            won: false
        }
    }

    pub fn new(brain: NeuralNet) -> Bot {
        let id = Uuid::new_v4().to_string();
        Bot {
            id,
            brain,
            normalized_fitness: 0.0,
            own_fitness: 0.0,
            round_index: 0,
            last_deaths: 0,
            won: false
        }
    }

    pub fn calc_own_fitness(&mut self, game_store: &GameStore) -> f32 {
        //maybe this has to be done after each round in the future
        let mut fitness = 0.0;
        let robot = game_store
            .robots
            .iter()
            .find_or_first(|r| r.user_name.eq(&self.id))
            .unwrap();
        self.last_deaths = robot.deaths;

        if self.won {
            fitness += 1.0;
            //TODO maybe change? 2 rounds per checkpoint too much?
            fitness += (2.0 * (game_store.highest_checkpoint+1) as f32)/(self.round_index as f32);
        } else { // is else a good idea or should they get a reward every time?
            fitness += robot.greatest_checkpoint_reached as f32 / CHECKPOINTS.len() as f32; //TODO!!!!! change to max num of checkpoints
        }

        fitness -= (robot.deaths as f32 / 2.0).exp(); //2 deaths is bad but oookay but from there on its really bad

        //fitness -= robot.hp as f32 / MAX_HP as f32;

        self.own_fitness = fitness.max(0.0);

        self.own_fitness
    }

    pub fn mutate(&mut self) {
        self.brain.mutate();
    }

    pub fn save_brain(&self) {
        self.brain.save();
    }

    pub fn play_cards(&self, gs: &GameStore, map: &Vec<TileSerialize>, checkpoints: &Vec<Position>) -> Vec<Card> {
        let me = gs.players.iter().find(|p| p.user_name.eq(&self.id)).unwrap();
        let mut cards = me.cards_in_hand.clone();
        let legal_amount = std::cmp::min(5, cards.len());
        let mut played_cards: Vec<Card> = Vec::new();

        if legal_amount < 1 {
            return played_cards;
        }
        
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

    fn play_card(&self, input: Vec<f32>) -> usize{
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