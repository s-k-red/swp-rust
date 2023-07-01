#![allow(dead_code)]
#![allow(unused_variables)]

use itertools::Itertools;

use crate::{
    config::{ CHECKPOINTS},
    datatypes::Position,
    serialization::TileSerialize,
    serialization_utils::load, components::GameStore,
};

use super::{bot::Bot, random_checkpoints};

pub struct Trainer {
    pub population: Vec<(Bot, GameStore)>,
    pub map: Vec<TileSerialize>,
    pub checkpoints: Vec<Position>,
}

impl Trainer {
    pub fn new() -> Trainer {
        let map = load();
        let cp = random_checkpoints();

        Trainer {
            population: Trainer::random_gen(&map, &cp),
            map,
            checkpoints: cp,
        }
    }
}
