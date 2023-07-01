use std::{io::{stdout, Write}, thread::{Thread, self}, fs};

use itertools::{Itertools, max};
use rand::Rng;
use uuid::Uuid;

use crate::{
    card_factory::create_card_deck,
    commands::TileEntity,
    components::{GameStore},
    datatypes::Position,
    serialization::TileSerialize,
    setup, config::CHECKPOINTS,
    config::POPULATION_SIZE,
    training::random_checkpoints,
};
use crate::training::crossover::crossover;
use crate::training::parent_selection::select_parents;

use super::{bot::Bot, trainer::Trainer, serializable_bot::SerializableBot};

impl Trainer {
    pub fn random_gen(map: &[TileSerialize], checkpoints: &[Position]) -> Vec<(Bot, GameStore)>{
        let mut pop = Vec::new();
        let m = map
            .iter()
            .map(|t| -> TileEntity { TileEntity::from(t.clone()) })
            .collect_vec();

        println!("Checkpoints: {:?}", checkpoints);

        for i in 0..POPULATION_SIZE {
                let cp = checkpoints.to_vec().clone();
                let bot = Bot::new_random();
                let mut gs = setup::convert(
                    m.clone(),
                vec![bot.id.clone()],
                create_card_deck(),
                cp[0],
                cp.len()-1,
                );
                gs.board
                    .add_checkpoints(cp);
                print!("\r{}/{}", i+1, POPULATION_SIZE);
                stdout().flush().unwrap();
                pop.push((bot, gs));
        }

        println!("\r");

        pop
    }

    pub fn gen_from_file(filepath: String) -> Vec<Bot> {
        // let serializable_bots =
        //     serde_json::from_str(fs::read_to_string(filepath).unwrap().as_str());

        todo!()
    }

    //version_data unix timestamp
    pub fn gen_to_file(gen: &[Bot], iteration: usize, version_date: u64){
        // let serializable_bots = gen.iter().map(|g|
        //     SerializableBot::from(g.clone())).collect_vec();

        let best_performing_bot = gen.iter().max_by(|a, b| a.normalized_fitness.total_cmp(&b.normalized_fitness)).unwrap();

        let gen_id = Uuid::new_v4();

        fs::create_dir(format!("gens/gen_{}_{}_{}", gen_id, iteration, version_date)).unwrap();

        // fs::write(format!("gens/gen_{}_{}_{}/gen_{}.json", gen_id, iteration, version_date, gen_id),
        //     serde_json::to_string(&serializable_bots).unwrap())
        //     .expect("failed to save gen");

        fs::write(format!("gens/gen_{}_{}_{}/best_bot.json", gen_id, iteration, version_date), serde_json::to_string(&SerializableBot::from(best_performing_bot.clone())).unwrap()).expect("failed to save best bot");
    }

    pub fn next_gen(
        last_gen: &mut Vec<(Bot, GameStore)>,
        map: &[TileSerialize],
        checkpoints: &[Position]
    ) -> Vec<(Bot, GameStore)> {
        let mut new_gen = Vec::new();

        calc_fitness(last_gen, &checkpoints.to_vec());
        let m = map
            .iter()
            .map(|t| -> TileEntity { TileEntity::from(t.clone()) })
            .collect_vec();

        let (max_fitness, _) = last_gen.iter().max_by(|(a1,b1),(a2,b2)|
            a1.own_fitness.total_cmp(&a2.own_fitness)).unwrap();

        let c = last_gen.iter().filter(|(b, g)| b.own_fitness == max_fitness.own_fitness).count();

        println!("Best fitness was: {} and {} bot(s) had this fitness, New Checkpoints: {:?}",
                 max_fitness.own_fitness,
                 c,
                 checkpoints
        );

        for i in (0..POPULATION_SIZE).step_by(2) {
            let parents = select_parents(last_gen.iter().map(|(b, g)| b).collect_vec().as_slice());
            let offspring = crossover(parents.as_slice());

            for mut b in offspring {
                b.round_index = 0;
                b.own_fitness = 0.0;
                b.normalized_fitness = 0.0;
                b.won = false;
                b.id = Uuid::new_v4().to_string();
                b.last_deaths = 0;
                let id = b.id.clone();
                b.mutate();
                let cp = checkpoints.to_vec().clone();
                let mut gs = setup::convert(
                    m.clone(),
                    vec![id],
                    create_card_deck(),
                    cp[0],
                    cp.len()-1,
                );
                gs.board.add_checkpoints(cp);
                print!("\r{}/{}", i+1, POPULATION_SIZE);
                stdout().flush().unwrap();
                new_gen.push((
                    b,
                    gs,
                ));
            }
        }

        println!("\r");

        new_gen
    }
}

fn calc_fitness(last_gen: &mut Vec<(Bot, GameStore)>, checkpoints: &Vec<Position>) {
    let mut sum = 0.0;

    for (bot, gs) in last_gen.iter_mut() {
        sum += bot.calc_own_fitness(gs, checkpoints);
    }

    for (ref mut bot, _) in last_gen {
        bot.normalized_fitness = bot.own_fitness / sum;
    }
}
