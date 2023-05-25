use itertools::Itertools;

use crate::{components::{GameStore, Card}, commands::{OnEntryTileEntity, TileEntity}, serialization::{TileEntitySerialize, TileSerialize}, datatypes::Position};

use super::bot::Bot;

pub fn get_inputs(bot: &Bot, gs: &GameStore, already_played_cards: &Vec<Card>, 
    map: &Vec<TileSerialize>, checkpoints: &Vec<Position>) -> Vec<f64> {
    let me = gs.players.iter().find(|p| p.user_name.eq(&bot.id)).unwrap();
    let my_robot = gs.robots.iter().find(|p| p.user_name.eq(&bot.id)).unwrap();
    let mut cards = me.cards_in_hand.clone();

    let mut input = Vec::new();

    for i in 0..9 {
        if cards.len() > i {
            input.push(f64::from(cards[i].id));
        } else {
            input.push(-1.0);
        }
    }

    for i in 0..5 {
        if already_played_cards.len() > i {
            input.push(f64::from(already_played_cards[i].id));
        } else {
            input.push(-1.0);
        }
    }

    input.push(f64::from(my_robot.position.x));
    input.push(f64::from(my_robot.position.y));
    input.push(f64::from(my_robot.facing_direction.ordinal()));
    input.push(f64::from(my_robot.hp));
    input.push(my_robot.safety_copy_amount as f64);
 
    let next_checkpoint = 
        checkpoints[my_robot.greatest_checkpoint_reached+1];

    input.push(next_checkpoint.x as f64);
    input.push(next_checkpoint.y as f64);


    for x in 0..12 {
        for y in 0..12 {
            let entities = map.iter()
                .filter(|m| m.position.x == x && m.position.y == y)
                .unique_by(|d| d.type_id)
                .collect_vec();

            for t in 0..4 {
                if entities.len() > t {
                    input.push(entities[t].direction.ordinal() as f64);
                    input.push(entities[t].type_id as f64);
                } else {
                    input.push(-1.0);
                    input.push(-1.0);
                }
            }
        }
    }

    for other_robot in 0..4 {
        input.push(-1.0); //x
        input.push(-1.0); //y
        input.push(-1.0); //direction
    }

    input
}