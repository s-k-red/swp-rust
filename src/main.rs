use crate::datatypes::datatypes::{Direction};

pub mod commands;
pub mod datatypes;
mod components;
mod game_states;
mod automaton;

fn main() {
    let direction = Direction::new(3);
    println!("{:?}", direction.turn(Direction::new(1)).to_position());
    println!("{:?}",direction)
}
