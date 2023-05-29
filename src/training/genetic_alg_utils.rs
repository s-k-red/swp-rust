use std::{io::{stdout, Write}, thread::{Thread, self}, fs};

use itertools::Itertools;
use rand::Rng;
use uuid::Uuid;

use crate::{
    card_factory::create_card_deck,
    commands::TileEntity,
    components::{GameStore},
    datatypes::Position,
    serialization::TileSerialize,
    setup, config::{CHECKPOINTS, PUPULATION_SIZE},
};

use super::{bot::Bot, trainer::Trainer, serializable_bot::SerializableBot};

impl Trainer {
    pub fn random_gen(map: &[TileSerialize]) -> Vec<(Bot, GameStore)>{
        let mut pop = Vec::new();
        print!("generating bots...");
        let mut threads = Vec::new();
        let m = map
            .iter()
            .map(|t| -> TileEntity { TileEntity::from(t.clone()) })
            .collect_vec();

        for _i in 0..PUPULATION_SIZE {
            let thread_map = m.clone();
            threads.push(thread::spawn(move || {
                let bot = Bot::new_random();
                let mut gs = setup::convert(
                    thread_map,
                vec![bot.id.clone()],
                create_card_deck(),
                CHECKPOINTS[0],
                1,
                );
                gs.board
                    .add_checkpoints(CHECKPOINTS.to_vec());
                print!(".");
                stdout().flush().unwrap();

                (bot, gs)
            }));
        }

        for thread in threads {
            let (bot, store) = thread.join().expect("Failed to join thread");
            pop.push((bot, store));
        }

        println!("DONE");

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
    ) -> Vec<(Bot, GameStore)> {
        let mut new_gen = Vec::new();
        let mut threads = Vec::new();
    
        calc_fitness(last_gen);
        let m = map
            .iter()
            .map(|t| -> TileEntity { TileEntity::from(t.clone()) })
            .collect_vec();
    
        for _bot in 0..last_gen.len() {
            let lg = last_gen.clone();
            let thread_m = m.clone();
            threads.push(thread::spawn(move || {
                let mut b = pick_bot(&lg).clone(); //crossover in the future?
                let id = b.id.clone();
                b.mutate();
                let mut gs = setup::convert(
                    thread_m,
                    vec![id],
                    create_card_deck(),
                    Position { x: 7, y: 7 },
                    1,
                );
                gs.board.add_checkpoints(vec![Position { x: 7, y: 7 }, Position { x: 7, y: 10 }]);
                print!(".");
                stdout().flush().unwrap();

                (b, gs)
            }));
            
        }

        for thread in threads {
            let (b, gs) = thread.join().expect("Failed to join thread");
            new_gen.push((
                b,
                gs,
            ));
        }
    
        println!("DONE");
    
        new_gen
    }
}

fn pick_bot(last_gen: &[(Bot, GameStore)]) -> &Bot {
    let mut rnd = rand::thread_rng();
    let mut index = 0;
    let mut r = rnd.gen::<f32>();

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
