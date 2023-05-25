use std::io::{stdout, Write};

use itertools::Itertools;
use rand::Rng;

use crate::{
    card_factory::create_card_deck,
    commands::TileEntity,
    components::{Board, GameStore, Player, Robot},
    datatypes::Position,
    serialization::TileSerialize,
    serialization_utils::load,
    setup,
};

use super::bot::Bot;

pub fn next_generation(
    last_gen: &mut Vec<(Bot, GameStore)>,
    map: &Vec<TileSerialize>,
) -> Vec<(Bot, GameStore)> {
    let mut new_gen = Vec::new();

    calc_fitness(last_gen);
    let mut stdout = stdout();
    let m = map
        .iter()
        .map(|t| -> TileEntity { TileEntity::from(t.clone()) })
        .collect_vec();

    for _bot in 0..last_gen.len() {
        print!(".");
        stdout.flush().unwrap();
        let mut b = pick_bot(last_gen).clone(); //crossover in the future?
        let id = b.id.clone();
        b.mutate();
        new_gen.push((
            b,
            setup::convert(
                m.clone(),
                vec![id],
                create_card_deck(),
                Position { x: 7, y: 7 },
                1,
            ),
        ));
    }
    //Hier fehlen die Checkpoints...
    println!("DONE");

    new_gen
}

fn pick_bot(last_gen: &[(Bot, GameStore)]) -> &Bot {
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

    for (bot, gs) in last_gen.iter_mut() {
        sum += bot.calc_own_fitness(gs);
    }

    for (ref mut bot, _) in last_gen {
        bot.normalized_fitness = bot.own_fitness / sum;
    }
}
