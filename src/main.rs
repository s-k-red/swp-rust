#[macro_use]
extern crate derive_builder;

use std::collections::HashMap;

use automaton::GameAutomaton;
use components::{Card, GameStore};

use crate::{
    config::{HIDDEN_LAYERS, INPUT_NODES},
    datatypes::{Direction, Position},
    neural_net::matrix_utils,
    training::trainer::Trainer,
};

mod automaton;
pub mod commands;
mod components;
mod config;
pub mod datatypes;
mod game_states;
mod neural_net;
mod resolve_movement;
pub mod scheduled_commands;
pub mod setup;
mod training;
fn main() {
    let direction = Direction::new(3);
    println!("{:?}", Position::from(direction.turn(Direction::new(1))));
    println!("{:?}", direction);

    let trainer = Trainer::new();
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
                    for (index, card) in cards.clone().iter_mut().enumerate() {
                        let _discard = std::mem::replace(
                            player.cards_in_hand.get_mut(index).unwrap(),
                            card.clone(),
                        );
                    }
                    player
                }
            }
            None => player,
        })
        .collect::<Vec<_>>();
    game_automaton.round_trip(&mut game_store);
    game_store
}
