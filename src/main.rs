#[macro_use]
extern crate derive_builder;

use crate::datatypes::{Direction, Position};

mod automaton;
pub mod commands;
mod components;
pub mod datatypes;
mod game_states;
mod resolve_movement;
pub mod setup;
fn main() {
    let direction = Direction::new(3);
    println!("{:?}", Position::from(direction.turn(Direction::new(1))));
    println!("{:?}", direction)
}
