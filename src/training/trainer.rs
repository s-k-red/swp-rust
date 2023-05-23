#![allow(dead_code)]
#![allow(unused_variables)]


use std::{collections::HashMap, thread, time::Duration};

use futures::future::join_all;
use itertools::Itertools;

use crate::{training::genetic_alg_utils, config::{GENERATIONS, PUPULATION_SIZE}, run_game, start_game, components::GameStore, setup, datatypes::Position, automaton::{self, GameAutomaton, AUTOMATON}, serialization_utils::load, card_factory::create_card_deck};

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
            let mut gs = setup::convert(load(), vec![bot.id.clone()], create_card_deck(), Position{x: 7, y: 7});
            gs.board.add_checkpoints(vec![Position{x:7, y:7}, Position{x:7, y:8}]);

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
            let mut me = store.robots.iter().find(|p| p.user_name.eq(&bot.id)).unwrap();
            println!("Round start for {} at pos x: {}, y: {}", bot.id, me.position.x, me.position.y);
            let mut played_cards = HashMap::new();
            played_cards.insert(bot.id.clone(), bot.play_cards(store));
            let res = run_game(played_cards, store, AUTOMATON);
            won = res.is_some();
            me = store.robots.iter().find(|p| p.user_name.eq(&bot.id)).unwrap();
            println!("Round done for {} at pos x: {}, y: {}", bot.id, me.position.x, me.position.y);
            thread::sleep(Duration::from_millis(1000));
        }
        
        todo!("start & play game until the end async");
    }
}