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
            input.extend_from_slice(calc_card_input(&cards[i]).as_slice());
        } else {
            for _bit in 0..7 {
                input.push(0.0);
            }
        }
    }

    for i in 0..5 {
        if already_played_cards.len() > i {
            input.extend_from_slice(calc_card_input(&already_played_cards[i]).as_slice());
        } else {
            for _bit in 0..7 {
                input.push(0.0);
            }
        }
    }

    for oh in 0..4 {
        if oh == my_robot.facing_direction.ordinal() {
            input.push(1.0);
            continue;
        }
        input.push(0.0);
    }
    input.push(my_robot.hp as f32);
    //input.push(my_robot.safety_copy_amount as f32);

    let next_checkpoint =
        checkpoints[my_robot.greatest_checkpoint_reached + 1];

    input.push((next_checkpoint.x - my_robot.position.x) as f32);
    input.push((next_checkpoint.y - my_robot.position.y) as f32);
    input.extend_from_slice(collect_tiles_as_relative_pos(my_robot, map).as_slice());

    input
}

fn collect_tiles_as_relative_pos(robot: &Robot, map: &[TileSerialize]) -> Vec<f32> {
    let positions = calc_tile_positions(&robot.position);
    let mut tiles = Vec::new();

    for pos in positions {
        // !!!!!!!!!!!!!!!!!!!! fill out of bounds positions with -1s
        if !(0..=11).contains(&pos.x) || !(0..=11).contains(&pos.y) {
            for _t in 0..4 {
                for _oh in 0..4 {
                    tiles.push(0.0);
                }
                for _oh in 0..=33 {
                    tiles.push(0.0);
                }
            }
            continue;
        }

        let entities = map.iter()
            .filter(|m| m.position.x == pos.x && m.position.y == pos.y)
            .unique_by(|d| d.type_id)
            .collect_vec();

        for t in 0..4 {
            if entities.len() > t {
                for oh in 0..4 {
                    if oh == entities[t].direction.ordinal() {
                        tiles.push(1.0);
                        continue;
                    }
                    tiles.push(0.0);
                }
                for oh in 0..=33 {
                    if entities[t].type_id == oh {
                        tiles.push(1.0);
                        continue;
                    }
                    tiles.push(0.0);
                }
            } else {
                for _oh in 0..4 {
                    tiles.push(0.0);
                }
                for _oh in 0..=33 {
                    tiles.push(0.0);
                }
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

fn calc_card_input(card: &Card) -> Vec<f32>{
    let mut res = Vec::new();

    //TURN LEFT
    if (70..411).step_by(20).contains(&(card.id as i32)){
        res.push(1.0);
    } else {
        res.push(0.0);
    }

    //TURN RIGHT
    if (80..421).step_by(20).contains(&(card.id as i32)){
        res.push(1.0);
    } else {
        res.push(0.0);
    }

    //U TURN
    if (10..61).step_by(10).contains(&(card.id as i32)){
        res.push(1.0);
    } else {
        res.push(0.0);
    }

    //BACK UP
    if (430..481).step_by(10).contains(&(card.id as i32)){
        res.push(1.0);
    } else {
        res.push(0.0);
    }

    //MOVE 1
    if (490..661).step_by(10).contains(&(card.id as i32)){
        res.push(1.0);
    } else {
        res.push(0.0);
    }

    //MOVE 2
    if (670..781).step_by(10).contains(&(card.id as i32)){
        res.push(1.0);
    } else {
        res.push(0.0);
    }

    //MOVE 3
    if (790..841).step_by(10).contains(&(card.id as i32)){
        res.push(1.0);
    } else {
        res.push(0.0);
    }

    res
}
