#![allow(dead_code)]
#![allow(unused_variables)]
pub(crate) mod automaton {
    use crate::{
        commands::commands::{RobotCommandAction, TileCommand, TileCommandAction},
        components::components::{GameStore, Robot},
        game_states::gamestates::GameState,
    };
    use std::collections::HashMap;

    struct Automaton {
        transitions: HashMap<GameState, GameState>,
    }

    trait StateAction {
        fn on_entry(&self, game_store: &mut GameStore);
    }

    impl StateAction for GameState {
        fn on_entry(&self, game_store: &mut GameStore) {
            match &self {
                GameState::Start => return,
                GameState::HandOutCards => todo!(),
                GameState::ExecuteCard(register_number) => {
                    let mut robots_with_cards = game_store
                        .robots
                        .iter_mut()
                        .filter_map(|robot| {
                            let card = robot.played_cards.get(*register_number as usize);
                            match card {
                                Some(_) => Some(robot),
                                None => None,
                            }
                        })
                        .collect::<Vec<&mut Robot>>();
                    robots_with_cards.sort_by_key(|robot| {
                        robot
                            .played_cards
                            .get(*register_number as usize)
                            .unwrap()
                            .id
                    });
                    for robot in robots_with_cards {
                        let commands = robot
                            .played_cards
                            .get(*register_number as usize)
                            .unwrap()
                            .commands
                            .clone();
                        for command in commands {
                            command.action(robot);
                        }
                    }
                }
                GameState::FactoryState(_, _) => {
                    let tile_commands = game_store
                        .tile_eintities
                        .get(&self)
                        .into_iter()
                        .flatten()
                        .map(|c| c.clone())
                        .collect::<Vec<TileCommand>>();
                    for tile_command in tile_commands {
                        tile_command.action(game_store);
                        movement_resolve(game_store);
                    }
                }
                GameState::RoundEnd => todo!(),
            }
        }
    }

    fn movement_resolve(game_store: &mut GameStore){
        
    }
}
