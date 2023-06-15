use rand::Rng;

use crate::{datatypes::Position, config::CHECKPOINTS};

pub mod trainer;
pub mod bot;
pub mod genetic_alg_utils;
mod input_builder;
mod async_trainer;
mod sync_trainer;
mod serializable_bot;
mod checkpoint_generator;
mod parent_selection;
mod crossover;

fn random_checkpoints() -> Vec<Position>{
    let mut rnd = rand::thread_rng();

    CHECKPOINTS[rnd.gen_range(0..CHECKPOINTS.len())].to_vec()
}

fn random_map() -> String {
    let maps = ["CanneryRow", "Exchange", "Island", "Maelstrom"];

    let mut rnd = rand::thread_rng();

    maps[rnd.gen_range(0..maps.len())].to_string()
}
