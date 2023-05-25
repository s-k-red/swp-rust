#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    config::{ CHECKPOINTS},
    datatypes::Position,
    serialization::TileSerialize,
    serialization_utils::load, components::GameStore,
};

use super::bot::Bot;

pub struct Trainer {
    pub population: Vec<(Bot, GameStore)>,
    pub map: Vec<TileSerialize>,
    pub checkpoints: Vec<Position>,
}

impl Trainer {
    pub fn new() -> Trainer {
        let map = load();

        Trainer {
            population: Trainer::random_gen(&map),
            map,
            checkpoints: CHECKPOINTS.to_vec(),
        }
    }
}
