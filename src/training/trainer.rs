#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{training::genetic_alg_utils, config::GENERATIONS};

use super::bot::Bot;

const PUPULATION_SIZE: i32 = 30;

pub struct Trainer {
    population: Vec<Bot>
}

impl Trainer {
    pub fn new() -> Trainer {
        let mut pop = Vec::new();

        for i in 0..PUPULATION_SIZE {
            print!("generating bot {} of {}..", i, PUPULATION_SIZE);
            let bot = Bot::new_random();
            println!("done!");

            pop.push(bot);
        }

        Trainer { population: pop }
    }

    pub async fn start_training(&mut self){
        todo!("async");

        self.run();
    }

    async fn run(&mut self){
        for generation in 0..GENERATIONS {
            let mut futures = Vec::new();

            for bot in &self.population {
                futures.push(self.play_bot(bot));
                todo!("start & play game until the end async");
            } 

            todo!("join futures");
    
            self.population = genetic_alg_utils::next_generation(&self.population)
        }
    }

    async fn play_bot(&self, bot: &Bot) -> Result<bool, bool>{
        Ok(true)
    }
}