#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap};

use crate::components::GameStore;

use super::bot::Bot;

const PUPULATION_SIZE: i32 = 30;

pub struct Trainer {
    population: HashMap<Bot, GameStore>
}

impl Trainer {
    pub fn new() -> Trainer {
        let mut pop = HashMap::new();

        pop.insert(Bot{}, GameStore{ robots: todo!(), players: todo!(), board: todo!(), card_deck: todo!(), winners: todo!() });

        Trainer { population: pop }
    }
}