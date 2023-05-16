#[macro_use]
extern crate derive_builder;

use std::{collections::HashMap, io, io::prelude::*};

use automaton::GameAutomaton;
use components::{Card, GameStore};
use serde_json::{Result, Value};

use crate::{
    commands::TileEntity,
    datatypes::{Direction, Position},
    game_states::GameState,
    neural_net::matrix_utils,
    serialization::TileEntitySerialize,
    training::{bot::Bot, trainer::Trainer},
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
mod serialization;
pub mod setup;
mod training;
fn main() {
    let direction = Direction::new(3);
    println!("{:?}", Position::from(direction.turn(Direction::new(1))));
    println!("{:?}", direction);

    //let trainer = Trainer::new();

    let t = TileEntity::Indirect(
        vec![GameState::FactoryState(1, game_states::FactoryState::Laser)],
        commands::IndirectTileEntity::Laser(Position { x: 3, y: 4 }, Direction::new(0), 3),
    );

    let v = serde_json::to_string(&t);

    println!("{}", v.ok().unwrap());
    //println!("{}", t.population.len());
    pause();
    // bot.save_brain();
    let inbounds = serde_json::to_string(&TileEntitySerialize::Inbounds).unwrap();

    let stadart_belt = serde_json::to_string(&TileEntitySerialize::Direct(
        vec![
            GameState::FactoryState(0, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(1, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(2, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(3, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(4, game_states::FactoryState::StandartBelt),
        ],
        serialization::RobotActionSerialize::Move,
    ))
    .unwrap();

    let express_belt = serde_json::to_string(&TileEntitySerialize::Direct(
        vec![
            GameState::FactoryState(0, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(1, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(2, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(3, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(4, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(0, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(1, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(2, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(3, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(4, game_states::FactoryState::ExpressBelt),
        ],
        serialization::RobotActionSerialize::Move,
    ))
    .unwrap();
    let standart_left_intersection = serde_json::to_string(&TileEntitySerialize::OnEntry(
        vec![
            GameState::FactoryState(0, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(1, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(2, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(3, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(4, game_states::FactoryState::StandartBelt),
        ],
        serialization::OnEntryTileEntitySerialize::WithDirection(
            serialization::RobotActionInPlaceSerialize::Turn,
            Direction::new(3),
        ),
    ))
    .unwrap();
    let standart_right_intersection = serde_json::to_string(&TileEntitySerialize::OnEntry(
        vec![
            GameState::FactoryState(0, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(1, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(2, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(3, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(4, game_states::FactoryState::StandartBelt),
        ],
        serialization::OnEntryTileEntitySerialize::WithDirection(
            serialization::RobotActionInPlaceSerialize::Turn,
            Direction::new(1),
        ),
    ))
    .unwrap();
    let standart_left_intersection = serde_json::to_string(&TileEntitySerialize::OnEntry(
        vec![
            GameState::FactoryState(0, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(1, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(2, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(3, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(4, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(0, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(1, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(2, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(3, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(4, game_states::FactoryState::ExpressBelt),
        ],
        serialization::OnEntryTileEntitySerialize::WithDirection(
            serialization::RobotActionInPlaceSerialize::Turn,
            Direction::new(3),
        ),
    ))
    .unwrap();
    let standart_right_intersection = serde_json::to_string(&TileEntitySerialize::OnEntry(
        vec![
            GameState::FactoryState(0, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(1, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(2, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(3, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(4, game_states::FactoryState::StandartBelt),
            GameState::FactoryState(0, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(1, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(2, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(3, game_states::FactoryState::ExpressBelt),
            GameState::FactoryState(4, game_states::FactoryState::ExpressBelt),
        ],
        serialization::OnEntryTileEntitySerialize::WithDirection(
            serialization::RobotActionInPlaceSerialize::Turn,
            Direction::new(1),
        ),
    ))
    .unwrap();
    let shover_0 = serde_json::to_string(&TileEntitySerialize::Direct(
        vec![GameState::FactoryState(
            0,
            game_states::FactoryState::Shover,
        )],
        serialization::RobotActionSerialize::Move,
    ))
    .unwrap();
    let shover_1 = serde_json::to_string(&TileEntitySerialize::Direct(
        vec![GameState::FactoryState(
            1,
            game_states::FactoryState::Shover,
        )],
        serialization::RobotActionSerialize::Move,
    ))
    .unwrap();
    let shover_2 = serde_json::to_string(&TileEntitySerialize::Direct(
        vec![GameState::FactoryState(
            2,
            game_states::FactoryState::Shover,
        )],
        serialization::RobotActionSerialize::Move,
    ))
    .unwrap();
    let shover_1_3 = serde_json::to_string(&TileEntitySerialize::Direct(
        vec![
            GameState::FactoryState(1, game_states::FactoryState::Shover),
            GameState::FactoryState(3, game_states::FactoryState::Shover),
        ],
        serialization::RobotActionSerialize::Move,
    ))
    .unwrap();
    let shover_0_2_4 = serde_json::to_string(&TileEntitySerialize::Direct(
        vec![
            GameState::FactoryState(0, game_states::FactoryState::Shover),
            GameState::FactoryState(2, game_states::FactoryState::Shover),
            GameState::FactoryState(4, game_states::FactoryState::Shover),
        ],
        serialization::RobotActionSerialize::Move,
    ))
    .unwrap();

    let crusher_2 = serde_json::to_string(&TileEntitySerialize::Direct(
        vec![GameState::FactoryState(1, game_states::FactoryState::Press)],
        serialization::RobotActionSerialize::InPlace(
            serialization::RobotActionInPlaceSerialize::Destroy,
        ),
    ))
    .unwrap();

    let crusher_1_3 = serde_json::to_string(&TileEntitySerialize::Direct(
        vec![
            GameState::FactoryState(0, game_states::FactoryState::Press),
            GameState::FactoryState(2, game_states::FactoryState::Press),
        ],
        serialization::RobotActionSerialize::InPlace(
            serialization::RobotActionInPlaceSerialize::Destroy,
        ),
    ))
    .unwrap();

    let shover_1_5 = serde_json::to_string(&TileEntitySerialize::Direct(
        vec![
            GameState::FactoryState(0, game_states::FactoryState::Press),
            GameState::FactoryState(4, game_states::FactoryState::Press),
        ],
        serialization::RobotActionSerialize::InPlace(
            serialization::RobotActionInPlaceSerialize::Destroy,
        ),
    ))
    .unwrap();
}

pub fn start_game(mut game_store: &mut GameStore) {
    GameAutomaton::hand_out_cards(game_store);
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
    GameAutomaton::hand_out_cards(&mut game_store);
    game_store
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
