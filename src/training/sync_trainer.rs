use std::{io::{stdout, Write}, collections::HashMap};

use futures::future::join_all;

use crate::{config::{GENERATIONS, ROUND_THRESHOLD}, training::genetic_alg_utils, serialization::TileSerialize, components::GameStore, datatypes::Position, start_game, run_game, automaton::AUTOMATON};

use super::{trainer::Trainer, bot::Bot};

impl Trainer {
    // pub async fn start_training(&mut self) {
    //     for generation in 0..GENERATIONS {
    //         println!("generating generation: {}", generation);
    //         stdout().flush().unwrap();
    //         if generation > 0 {
    //             self.population =
    //                 Trainer::next_gen(&mut self.population.clone(), &self.map);
    //         }
    //         self.run().await;
    //     }
    // }

    // async fn run(&mut self) {
    //     let mut futs = Vec::new();

    //     for (bot, store) in &mut self.population {
    //         futs.push(Trainer::play_bot(bot, store, &self.map, &self.checkpoints));
    //     }

    //     join_all(futs).await;
    // }

    // async fn play_bot(
    //     bot: &mut Bot,
    //     store: &mut GameStore,
    //     map: &Vec<TileSerialize>,
    //     checkpoints: &Vec<Position>,
    // ) -> Result<bool, ()> {
    //     start_game(store);
    //     let mut won = false;
    //     while !won {
    //         let mut played_cards = HashMap::new();
    //         played_cards.insert(bot.id.clone(), bot.play_cards(store, map, checkpoints));
    //         let res = run_game(played_cards, store, AUTOMATON);
    //         won = res.is_some();
    //         if won {
    //             bot.won = res.unwrap().contains(&bot.id);
    //         }
    //         bot.round_index += 1;
            
    //         if bot.round_index > ROUND_THRESHOLD {
    //             break;
    //         }
    //     }

        

    //     // println!(
    //     //     "Bot {} won {} in {} rounds with {} deaths and {} checkpoints reached",
    //     //     bot.id,
    //     //     bot.won,
    //     //     bot.round_index,
    //     //     store.robots.first().unwrap().deaths,
    //     //     store.robots.first().unwrap().greatest_checkpoint_reached+1
    //     // );

    //     Ok(true)
    // }
}