use std::{thread, collections::HashMap, time::{SystemTime, UNIX_EPOCH}};
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};
use itertools::Itertools;

use crate::{config::{GENERATIONS, ROUND_THRESHOLD, POPULATION_SIZE}, training::genetic_alg_utils, start_game, automaton::AUTOMATON, run_game};
use crate::config::SAVE_GEN_INTERVAL;
use crate::training::random_checkpoints;

use super::trainer::Trainer;


impl Trainer {
    pub fn start_training_async(mut self) {
        let mut avg_death = 300.0;
        let mut gen = 0;

        // for gen in 0..GENERATIONS {
        //
        // }

        while gen <= GENERATIONS {
            if gen > 0 {
                if gen % SAVE_GEN_INTERVAL == 0 {
                    let best = self.population.iter()
                        .map(|(b, g)| b)
                        .max_by(|a, b| a.normalized_fitness.total_cmp(&b.normalized_fitness))
                        .unwrap().clone();

                    Trainer::gen_to_file(vec![best].as_slice(), gen, SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
                }

                let filtered_pop = &mut self.population;
                // let filtered_pop = &mut self.population.into_iter()
                // .filter(|p| p.0.won)
                // .collect_vec();
                self.checkpoints = random_checkpoints();
                self.population = Trainer::next_gen(filtered_pop, &self.map, &self.checkpoints);
            }

            let mut threads = Vec::new();
            let mut pop = Vec::new();
            let mut ids = Vec::new();

            while let Some(element) = self.population.first().cloned() {
                pop.push(element.clone());
                ids.push(element.0.id);
                self.population.drain(..1);
            }

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
                        //println!("Run game {}", bot.id);
                        let res = run_game(played_cards, &mut store, AUTOMATON);
                        won = res.is_some();
                        if won {
                            bot.won = res.unwrap().contains(&bot.id);
                        }
                        bot.round_index += 1;

                        if bot.round_index > ROUND_THRESHOLD {
                            return (bot, store);
                        }
                    }

                    (bot, store)
                });

                threads.push(thread);
            }

            for (i, thread) in threads.into_iter().enumerate() {
                print!("\rJoin Bot {} Nr {}/{}", ids[i], i, POPULATION_SIZE);
                stdout().flush().unwrap();

                let (bot, store) = thread.join().expect("Failed to join thread");
                //print!("| done with {} rounds", bot.round_index);
                self.population.push((bot, store));
            }

            avg_death = (self.population.iter().map(|p| p.1.robots[0].deaths as usize).collect::<Vec<usize>>().iter().sum::<usize>() as f32) / (self.population.len() as f32);

            println!("\rGeneration {} done: Total rounds {}, w/l {}/{}, avg deaths {:.3}, avg rounds {:.3}",
                     gen,
                     self.population.iter().map(|p| p.0.round_index).collect::<Vec<usize>>().iter().sum::<usize>(),
                     self.population.iter().filter(|p| p.0.won).count(),
                     self.population.iter().filter(|p| !p.0.won).count(),
                     avg_death,
                     (self.population.iter().map(|p| p.0.round_index).collect::<Vec<usize>>().iter().sum::<usize>() as f32) / (self.population.len() as f32),
            );

            gen+=1;
        }


        Trainer::gen_to_file(
            &self.population.iter().map(|p| p.0.clone()).collect_vec(),
            GENERATIONS,
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
    }
}
