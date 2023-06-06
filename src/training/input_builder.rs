use itertools::Itertools;

use crate::{components::{GameStore, Card}, serialization::TileSerialize, datatypes::Position};

use super::bot::Bot;

pub fn get_inputs(bot: &Bot, gs: &GameStore, already_played_cards: &Vec<Card>, 
    map: &[TileSerialize], checkpoints: &[Position]) -> Vec<f32> {
    let me = gs.players.iter().find(|p| p.user_name.eq(&bot.id)).unwrap();
    let my_robot = gs.robots.iter().find(|p| p.user_name.eq(&bot.id)).unwrap();
    let cards = me.cards_in_hand.clone();

    let mut input = Vec::new();

    for i in 0..9 {
        if cards.len() > i {
            input.push((cards[i].id as f32) / 10.0);
        } else {
            input.push(-1.0);
        }
    }

    for i in 0..5 {
        if already_played_cards.len() > i {
            input.push((already_played_cards[i].id as f32) / 10.0);
        } else {
            input.push(-1.0);
        }
    }

    input.push(my_robot.position.x as f32);
    input.push(my_robot.position.y as f32);
    input.push(my_robot.facing_direction.ordinal() as f32);
    input.push(my_robot.hp as f32);
    input.push(my_robot.safety_copy_amount as f32);
 
    let next_checkpoint = 
        checkpoints[my_robot.greatest_checkpoint_reached+1];

    input.push(next_checkpoint.x as f32);
    input.push(next_checkpoint.y as f32);


    for x in 0..12 {
        for y in 0..12 {
            let entities = map.iter()
                .filter(|m| m.position.x == x && m.position.y == y)
                .unique_by(|d| d.type_id)
                .collect_vec();

            for t in 0..4 {
                if entities.len() > t {
                    input.push(entities[t].direction.ordinal() as f32);
                    input.push(entities[t].type_id as f32);
                } else {
                    input.push(-1.0);
                    input.push(-1.0);
                }
            }
        }
    }

    for _other_robot in 0..4 {
        input.push(-1.0); //x
        input.push(-1.0); //y
        input.push(-1.0); //direction
    }

    input
}