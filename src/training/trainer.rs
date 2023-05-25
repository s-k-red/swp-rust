#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::HashMap,
    io::{stdout, Write},
    thread,
    time::Duration,
};

use futures::future::join_all;
use itertools::Itertools;

use crate::{
    automaton::{self, GameAutomaton, AUTOMATON},
    card_factory::create_card_deck,
    commands::TileEntity,
    components::GameStore,
    config::{GENERATIONS, PUPULATION_SIZE, CHECKPOINTS, ROUND_THRESHOLD},
    datatypes::Position,
    run_game,
    serialization::TileSerialize,
    serialization_utils::load,
    setup, start_game,
    training::genetic_alg_utils,
};

use super::bot::Bot;

pub struct Trainer {
    pub population: Vec<(Bot, GameStore)>,
    pub map: Vec<TileSerialize>,
    pub checkpoints: Vec<Position>,
}

impl Trainer {
    pub fn new() -> Trainer {
        let mut pop = Vec::new();

        let map = load();
        print!("generating bots");
        let mut stdout = stdout();
        let m = map
            .iter()
            .map(|t| -> TileEntity { TileEntity::from(t.clone()) })
            .collect_vec();

        for i in 0..PUPULATION_SIZE {
            print!(".");
            stdout.flush().unwrap();
            let bot = Bot::new_random();
            let mut gs = setup::convert(
                m.clone(),
                vec![bot.id.clone()],
                create_card_deck(),
                CHECKPOINTS[0],
                1,
            );
            gs.board
                .add_checkpoints(CHECKPOINTS.to_vec());

            pop.push((bot, gs));
        }

        println!("DONE");

        Trainer {
            population: pop,
            map,
            checkpoints: CHECKPOINTS.to_vec(),
        }
    }

    pub async fn start_training(&mut self) {
        for generation in 0..GENERATIONS {
            println!("generating generation: {}", generation);
            stdout().flush().unwrap();
            if generation > 0 {
                self.population =
                    genetic_alg_utils::next_generation(&mut self.population.clone(), &self.map);
            }
            self.run().await;
        }
    }

    async fn run(&mut self) {
        let mut futs = Vec::new();

        for (bot, store) in &mut self.population {
            futs.push(Trainer::play_bot(bot, store, &self.map, &self.checkpoints));
        }

        join_all(futs).await;
    }

    async fn play_bot(
        bot: &mut Bot,
        store: &mut GameStore,
        map: &Vec<TileSerialize>,
        checkpoints: &Vec<Position>,
    ) -> Result<bool, ()> {
        start_game(store);
        let mut won = false;
        while !won {
            let mut played_cards = HashMap::new();
            played_cards.insert(bot.id.clone(), bot.play_cards(store, map, checkpoints));
            let res = run_game(played_cards, store, AUTOMATON);
            won = res.is_some();
            if won {
                bot.won = res.unwrap().contains(&bot.id);
            }
            bot.round_index += 1;
            
            if bot.round_index > ROUND_THRESHOLD {
                break;
            }
        }

        

        println!(
            "Bot {} won {} in {} rounds with {} deaths and {} checkpoints reached",
            bot.id,
            bot.won,
            bot.round_index,
            store.robots.first().unwrap().deaths,
            store.robots.first().unwrap().greatest_checkpoint_reached+1
        );

        Ok(true)
    }
}
