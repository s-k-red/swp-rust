#[macro_use]
extern crate derive_builder;

use std::{collections::HashMap, io, io::prelude::*};

use automaton::{GameAutomaton, AUTOMATON_SIZE};
use components::{Card, GameStore};

use futures::executor::block_on;

use training::trainer::Trainer;

use crate::components::Board;

mod automaton;
pub mod commands;
mod components;
mod config;
pub mod datatypes;
mod game_states;
mod neural_net;
mod resolve_movement;
pub mod scheduled_commands;
mod serialization;
mod serialization_utils;
pub mod setup;
mod training;
mod card_factory;
fn main() {
    let mut trainer = Trainer::new();

    block_on(trainer.start_training());
}
pub fn start_game(game_store: &mut GameStore) {
    GameAutomaton::<AUTOMATON_SIZE>::hand_out_cards(game_store);
}

pub fn run_game<const N: usize>(
    cards_played: HashMap<String, Vec<Card>>,
    mut game_store: GameStore,
    game_automaton: GameAutomaton<{ N }>,
) -> (GameStore, Option<Vec<String>>) {
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
    let winners = game_automaton.round_trip(&mut game_store);
    GameAutomaton::<AUTOMATON_SIZE>::hand_out_cards(&mut game_store);
    (game_store, winners)
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
