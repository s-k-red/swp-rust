#[macro_use]
extern crate derive_builder;

use std::collections::HashMap;

use automaton::GameAutomaton;
use components::{Card, GameStore};

use crate::datatypes::{Direction, Position};

mod automaton;
pub mod commands;
mod components;
pub mod datatypes;
mod game_states;
mod resolve_movement;
mod neural_net;
pub mod setup;
fn main() {
    let direction = Direction::new(3);
    println!("{:?}", Position::from(direction.turn(Direction::new(1))));
    println!("{:?}",direction);
}

pub fn start_game(mut game_store: GameStore) -> GameStore {
    GameAutomaton::hand_out_cards(&mut game_store);
    game_store
}
pub fn run_game(
    cards_played: HashMap<String, Vec<Card>>,
    mut game_store: GameStore,
    game_automaton: GameAutomaton,
) -> GameStore {
    game_store.players = game_store
        .players
        .into_iter()
        .map(|mut player| match cards_played.get(&player.user_name) {
            Some(cards) => {
                if player.cards_in_hand.is_empty() {
                    player.cards_in_hand = cards.clone();
                    player
                } else {
                    for card in cards {}
                    player
                }
            }
            None => player,
        })
        .collect::<Vec<_>>();
    game_store
}
