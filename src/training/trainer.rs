#![allow(dead_code)]
#![allow(unused_variables)]


use std::collections::HashMap;

use futures::future::join_all;
use itertools::Itertools;

use crate::{training::genetic_alg_utils, config::{GENERATIONS, PUPULATION_SIZE}, run_game, start_game, components::GameStore, setup, datatypes::Position, automaton::{self, GameAutomaton, AUTOMATON}, serialization_utils::load};

use super::bot::Bot;



pub struct Trainer {
    pub population: Vec<(Bot, GameStore)>
}

impl Trainer {
    pub fn new() -> Trainer {
        let mut pop = Vec::new();

        for i in 0..PUPULATION_SIZE {
            print!("generating bot {} of {}..", i, PUPULATION_SIZE);
            let bot = Bot::new_random();
            println!("done!");
            let gs = setup::convert(load(), vec![bot.id.clone()], Vec::new(), Position{x: 3, y: 4});

            pop.push((bot, gs));
        }

        Trainer { population: pop }
    }

    pub async fn start_training(&mut self){
        for generation in 0..GENERATIONS {
            println!("Generation: {}", generation);
            self.run().await;
        }
    }

    async fn run(&mut self){
        let mut futs = Vec::new();

        for (bot, store) in &mut self.population {
            futs.push(Trainer::play_bot(bot, store));
        }  

        join_all(futs).await;

        self.population = genetic_alg_utils::next_generation(&mut self.population)
    }

    async fn play_bot(bot: &mut Bot, store: &mut GameStore) -> Result<bool, ()>{
        println!("Start game for {}", bot.id);
        start_game(store);
        let mut won = false;

        while !won {
            println!("Play round for {}", bot.id);
            let mut played_cards = HashMap::new();
            played_cards.insert(bot.id.clone(), bot.play_cards(store));
            won = run_game(played_cards, store.clone(), AUTOMATON).1.is_some()
        }
        
        todo!("start & play game until the end async");
    }
}