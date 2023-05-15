use rand::Rng;

use crate::{components::{GameStore, Robot, Board, Player}, datatypes::Position};

use super::bot::Bot;

pub fn next_generation(mut last_gen: &mut Vec<(Bot, GameStore)>) -> Vec<(Bot, GameStore)>{
    let mut new_gen = Vec::new();

    calc_fitness(last_gen);
    for _bot in 0..last_gen.len() {
        let mut b = pick_bot(&last_gen).clone(); //crossover in the future?
        b.mutate();
        new_gen.push((b, GameStore { robots: vec![ Robot::new(b.id.clone(), Position { x: 0, y: 7 })], players: vec![Player::new(b.id.clone())], board: Board::new(Vec::new()), card_deck: Vec::new(), winners: Vec::new() })); 
    }

    new_gen
}

fn pick_bot(last_gen: &Vec<(Bot, GameStore)>) -> &Bot{
    let mut rnd = rand::thread_rng();
    let mut index = 0;
    let mut r = rnd.gen::<f64>();

    while r > 0.0 {
        r -= last_gen[index].0.normalized_fitness;
        index += 1;
    }

    index -= 1;

    &last_gen[index].0
}

fn calc_fitness(last_gen: &mut Vec<(Bot, GameStore)>) {
    let mut sum = 0.0;

    for (bot, gs) in last_gen { 
        sum += bot.calc_own_fitness(gs);
    }

    for mut bot in last_gen {
        bot.0.normalized_fitness = bot.0.own_fitness / sum;
    }
}