#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;

use crate::{
    commands::{execute, RobotActionInPlace, ScheduledActions, TileCommand, TransitionLocks},
    components::{Board, Card, Player, Robot},
    game_states::GameState, resolve_movement::resolve_movement,
};


trait StateAction {
    fn on_entry(&self, robots: &mut [Robot], board: &Board, players: &mut [Player]);
}

impl StateAction for GameState {
    fn on_entry(&self, robots: &mut [Robot], board: &Board, players: &mut [Player]) {
        match &self {
            GameState::Start => (),
            GameState::HandOutCards => todo!(),
            GameState::ExecuteCard(register_number) => {
                let mut cards = players
                    .iter()
                    .filter_map(|player| {
                        player
                            .cards_played
                            .get(*register_number)
                            .map(|c| (player, c))
                    })
                    .collect::<Vec<(&Player, &Card)>>();
                cards.sort_by_key(|(_, card)| card.id);
                for (player, card) in cards {
                    for cmd in &card.commands {
                        let robot_actions = vec![robots
                            .iter_mut()
                            .find(|robot| robot.user_name == player.user_name)
                            .map(|robot| {
                                if robot.user_name == player.user_name {
                                    let mut actions = ScheduledActions::new(robot);
                                    actions.insert_and_convert(cmd.clone());
                                }
                                ScheduledActions::new(robot)
                            })
                            .unwrap()];
                        let robot_actions = resolve_movement(
                            robot_actions,
                            board,
                            TransitionLocks::new(&board.walls),
                        );
                        for robot_action in robot_actions {
                            execute(robot_action);
                        }
                    }
                }
            }
            GameState::FactoryState(_, _) => {
                let robot_actions = calculate_actions_from_tile_entities(self, robots, board);
                let robot_actions =
                    resolve_movement(robot_actions, board, TransitionLocks::new(&board.walls));
                for robot_action in robot_actions {
                    execute(robot_action);
                }
            }
            GameState::RoundEnd => todo!(),
        }
    }
}

fn calculate_actions_from_tile_entities<'a>(
    game_state: &GameState,
    robots: &'a mut [Robot],
    board: &Board,
) -> Vec<ScheduledActions<'a>> {
    let active_tile_commands = board
        .tile_eintities
        .get(game_state)
        .unwrap_or(&vec![])
        .clone();
    let mut action_map = robots
        .iter_mut()
        .map(|robot| (robot.position, ScheduledActions::new(robot)))
        .collect::<HashMap<_, _>>();

    for tile_command in active_tile_commands {
        match tile_command {
            TileCommand::FromRobotCommand(pos, cmd) => {
                if let Some(actions) = action_map.get_mut(&pos) {
                    actions.insert_and_convert(cmd);
                }
            }
            TileCommand::Laser(pos, dir, intensity) => {
                let laser_positions = board.all_pos_inbounds_in_direction_until_blocked(pos, dir);
                for pos in laser_positions {
                    match action_map.get_mut(&pos) {
                        Some(actions) => {
                            actions.insert(RobotActionInPlace::TakeDamage(intensity));
                            break;
                        }
                        None => continue,
                    }
                }
            }
        };
    }
    action_map.into_iter().map(|entry| entry.1).collect()
}
