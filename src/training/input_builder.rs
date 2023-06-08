use std::fs;
use itertools::Itertools;
use num::traits::Pow;

use crate::{components::{GameStore, Card}, serialization::TileSerialize, datatypes::Position};
use crate::commands::TileEntity;
use crate::components::Robot;

use super::bot::Bot;

pub fn get_inputs(bot: &Bot, gs: &GameStore, already_played_cards: &Vec<Card>,
                  map: &[TileSerialize], checkpoints: &[Position]) -> Vec<f32> {
    let me = gs.players.iter().find(|p| p.user_name.eq(&bot.id)).unwrap();
    let my_robot = gs.robots.iter().find(|p| p.user_name.eq(&bot.id)).unwrap();
    let cards = me.cards_in_hand.clone();

    let mut input = Vec::new();

    for i in 0..9 {
        if cards.len() > i {
            for bit in 1..11 {
                if cards[i].id as i32 & 2.pow(bit as u32) > 0 {
                    input.push(1.0);
                } else {
                    input.push(0.0);
                }
            }
        } else {
            for _bit in 0..10 {
                input.push(-1.0);
            }
        }
    }

    for i in 0..5 {
        if already_played_cards.len() > i {
            for bit in 1..11 {
                input.push((already_played_cards[i].id as i32 & 2.pow(bit as u32)) as f32);
            }
        } else {
            for _bit in 0..10 {
                input.push(-1.0);
            }
        }
    }

    input.push(my_robot.facing_direction.ordinal() as f32);
    input.push(my_robot.hp as f32);
    input.push(my_robot.safety_copy_amount as f32);

    let next_checkpoint =
        checkpoints[my_robot.greatest_checkpoint_reached + 1];

    input.push((next_checkpoint.x - my_robot.position.x) as f32);
    input.push((next_checkpoint.y - my_robot.position.y) as f32);
    input.extend_from_slice(collect_tiles_as_relative_pos(my_robot, map).as_slice());

    // for _other_robot in 0..4 {
    //     input.push(-1.0); //x
    //     input.push(-1.0); //y
    //     input.push(-1.0); //direction
    // }

    input
}

fn collect_tiles_as_relative_pos(robot: &Robot, map: &[TileSerialize]) -> Vec<f32> {
    let positions = calc_tile_positions(&robot.position);
    let mut tiles = Vec::new();

    for pos in positions {
        // !!!!!!!!!!!!!!!!!!!! fill out of bounds positions with -1s
        if !(0..=11).contains(&pos.x) || !(0..=11).contains(&pos.y) {
            for _t in 0..4 {
                tiles.push(-1.0);
                tiles.push(-1.0);
            }
            continue;
        }

        let entities = map.iter()
            .filter(|m| m.position.x == pos.x && m.position.y == pos.y)
            .unique_by(|d| d.type_id)
            .collect_vec();

        for t in 0..4 {
            if entities.len() > t {
                tiles.push(entities[t].direction.ordinal() as f32);
                tiles.push(entities[t].type_id as f32);
            } else {
                tiles.push(-1.0);
                tiles.push(-1.0);
            }
        }
    }

    tiles
}

//Calculate view point of 7x7 or radius of 3 around the robot
fn calc_tile_positions(pos: &Position) -> Vec<Position> {
    let mut positions = Vec::new();

    for x in pos.x - 3..pos.x + 4 { //+4 cause exclusive
        for y in pos.y - 3..pos.y + 4 {
            positions.push(Position { x, y });
        }
    }

    positions
}
