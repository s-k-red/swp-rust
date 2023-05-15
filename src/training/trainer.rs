#![allow(dead_code)]
#![allow(unused_variables)]


use futures::future::join_all;
use itertools::Itertools;

use crate::{training::genetic_alg_utils, config::GENERATIONS, run_game, start_game, components::GameStore, setup};

use super::bot::Bot;

const PUPULATION_SIZE: i32 = 30;

pub struct Trainer {
    population: Vec<(Bot, GameStore)>
}

impl Trainer {
    pub fn new() -> Trainer {
        let mut pop = Vec::new();

        for i in 0..PUPULATION_SIZE {
            print!("generating bot {} of {}..", i, PUPULATION_SIZE);
            let bot = Bot::new_random();
            println!("done!");
            let gs = setup::convert(board, vec![bot.id.clone()], card_deck, starting_pos);

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

            for (_, store) in &mut self.population {
                futs.push(Trainer::play_bot(store));
            }  

            join_all(futs).await;
    
            self.population = genetic_alg_utils::next_generation(&mut self.population)
    }

    async fn play_bot(store: &mut GameStore) -> Result<bool, ()>{
        start_game(store);
        todo!("start & play game until the end async");
        run_game(cards_played, game_store, game_automaton)
    }
}