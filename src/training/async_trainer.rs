use std::{thread, collections::HashMap};

use crate::{config::{GENERATIONS, ROUND_THRESHOLD}, training::genetic_alg_utils, start_game, automaton::AUTOMATON, run_game};

use super::trainer::Trainer;


impl Trainer {
    // pub fn start_training_async(mut self){
    //     for gen in 0..GENERATIONS {
    //         println!("generating generation: {}", gen);

    //         if gen > 0 {
    //             genetic_alg_utils::next_generation(self.population.clone(), &self.map);
    //         }

    //         let mut threads = Vec::new();
    //         let pop = self.population.clone();

    //         for (mut bot, mut store) in pop {
    //             let thread = thread::spawn(|| {
    //                 start_game(&mut store);
    //                 let mut won = false;
    //                 while !won {
    //                     let mut played_cards = HashMap::new();
    //                     played_cards.insert(bot.id.clone(), bot.play_cards(&store, &self.map, &self.checkpoints));
    //                     let res = run_game(played_cards, &mut store, AUTOMATON);
    //                     won = res.is_some();
    //                     if won {
    //                         bot.won = res.unwrap().contains(&bot.id);
    //                     }
    //                     bot.round_index += 1;
                        
    //                     if bot.round_index > ROUND_THRESHOLD {
    //                         break;
    //                     }
    //                 }
                    

    //                 println!(
    //                     "Bot {} won {} in {} rounds with {} deaths and {} checkpoints reached",
    //                     bot.id,
    //                     bot.won,
    //                     bot.round_index,
    //                     store.robots.first().unwrap().deaths,
    //                     store.robots.first().unwrap().greatest_checkpoint_reached+1
    //                 );
    //                 self.population.push((bot, store));
    //             });

    //             threads.push(thread);
    //         }
    //     }
    // }
}