#![allow(dead_code)]
#![allow(unused_variables)]

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
}