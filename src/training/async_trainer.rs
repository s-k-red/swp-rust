use std::{thread, collections::HashMap};

use crate::{config::{GENERATIONS, ROUND_THRESHOLD}, training::genetic_alg_utils, start_game, automaton::AUTOMATON, run_game};

use super::trainer::Trainer;


impl Trainer {
    pub fn start_training_async(mut self) {
        for gen in 0..GENERATIONS {
            println!("generating generation: {}", gen);
    
            if gen > 0 {
                self.population = genetic_alg_utils::next_generation(&mut self.population, &self.map);
            }
        
            let mut threads = Vec::new();
            let pop = self.population.clone();
            self.population.clear();
            let map = self.map.clone();
            let checkpoints = self.checkpoints.clone(); // Clone the checkpoints vector
        
            for (mut bot, mut store) in pop {
                let map = map.clone();
                let checkpoints = checkpoints.clone(); // Clone the checkpoints vector for each thread
        
                let thread = thread::spawn(move || {
                    start_game(&mut store);
                    let mut won = false;
                    while !won {
                        let mut played_cards = HashMap::new();
                        played_cards.insert(bot.id.clone(), bot.play_cards(&store, &map, &checkpoints)); // Use the cloned checkpoints vector
                        let res = run_game(played_cards, &mut store, AUTOMATON);
                        won = res.is_some();
                        if won {
                            bot.won = res.unwrap().contains(&bot.id);
                        }
                        bot.round_index += 1;
        
                        if bot.round_index > ROUND_THRESHOLD {
                            break;
                        }
                    }
        
                    let first_robot = store.robots.first().unwrap();
                    println!(
                        "Bot {} won {} in {} rounds with {} deaths and {} checkpoints reached",
                        bot.id,
                        bot.won,
                        bot.round_index,
                        first_robot.deaths,
                        first_robot.greatest_checkpoint_reached + 1
                    );
                    (bot, store)
                });
        
                threads.push(thread);
            }
        
            for thread in threads {
                let (bot, store) = thread.join().expect("Failed to join thread");
                self.population.push((bot, store));
            }
        }
    }
}