use crate::datatypes::{Direction, Position};

pub mod commands;
pub mod datatypes;
mod components;
mod game_states;
mod automaton;

fn main() {
    let direction = Direction::new(3);
    println!("{:?}", Position::from(direction.turn(Direction::new(1))));
    println!("{:?}",direction)
}
