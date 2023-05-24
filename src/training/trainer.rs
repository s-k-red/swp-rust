#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, thread, time::Duration, io::{stdout, Write}};

use futures::future::join_all;
use itertools::Itertools;

use crate::{
    automaton::{self, GameAutomaton, AUTOMATON},
    card_factory::create_card_deck,
    components::GameStore,
    config::{GENERATIONS, PUPULATION_SIZE},
    datatypes::Position,
    run_game,
    serialization_utils::load,
    setup, start_game,
    training::genetic_alg_utils, serialization::TileSerialize, commands::TileEntity,
};

use super::bot::Bot;

pub struct Trainer {
    pub population: Vec<(Bot, GameStore)>,
    pub map: Vec<TileSerialize>,
    pub checkpoints: Vec<(usize, Position)>
}

impl Trainer {
    pub fn new() -> Trainer {
        let mut pop = Vec::new();

        let map = load();
        print!("generating bots");
        let mut stdout = stdout();
        let m = map.iter().map(|t| -> TileEntity {TileEntity::from(t.clone())}).collect_vec();

        for i in 0..PUPULATION_SIZE {
            print!(".");
            stdout.flush().unwrap();
            let bot = Bot::new_random();
            let mut gs = setup::convert(
                m.clone(),
                vec![bot.id.clone()],
                create_card_deck(),
                Position { x: 7, y: 7 },
            );
            gs.board
                .add_checkpoints(vec![Position { x: 7, y: 7 }, Position { x: 7, y: 8 }]);

            pop.push((bot, gs));
        }

        println!("DONE");

        Trainer { population: pop, map, checkpoints: vec![(0, Position { x: 7, y: 7 }), (1,  Position { x: 7, y: 8 })] }
    }

    pub async fn start_training(&mut self) {
        for generation in 0..GENERATIONS {
            print!("generating generation: {}", generation);
            stdout().flush().unwrap();
            if generation > 0 {
                self.population = genetic_alg_utils::next_generation(&mut self.population, &self.map);
            }
            println!();
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

    async fn play_bot(bot: &mut Bot, store: &mut GameStore, 
        map: &Vec<TileSerialize>, checkpoints: &Vec<(usize, Position)>) -> Result<bool, ()> {
        start_game(store);
        let mut won = false;
        let mut round_index = 0;
        while !won {
            let mut played_cards = HashMap::new();
            played_cards.insert(bot.id.clone(), bot.play_cards(store, map, checkpoints));
            let res = run_game(played_cards, store, AUTOMATON);
            won = res.is_some();
            round_index += 1;
            //thread::sleep(Duration::from_millis(1000));
        }

        println!("Bot {} won in {} rounds with {} deaths", bot.id, round_index, store.robots.first().unwrap().deaths);

        Ok(true)
    }
}
