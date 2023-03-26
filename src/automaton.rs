#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;

use crate::{
    commands::{RobotAction, RobotCommand, ScheduledAction, ScheduledRobotCommand, TileCommand}, components::{Robot, Player, Card, Board}, game_states::{GameState},
};

trait StateAction {
    fn on_entry(&self, robots:  &mut [Robot], board: &Board, players: &mut [Player]);
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
                                    return ScheduledRobotCommand (robot, cmd.clone() );
                                }
                                ScheduledRobotCommand (robot,RobotCommand::Absolute(RobotAction::None))
                            })
                            .map(ScheduledAction::from)
                            .unwrap()];
                        let robot_actions = resolve_movement(robot_actions, board);
                        for mut robot_action in robot_actions {
                            robot_action.execute();
                        }
                    }
                }
            }
            GameState::FactoryState(_,_) => {
                let robot_actions = calculate_actions_from_tile_entities(self, robots, board);
                for mut robot_action in robot_actions {
                    robot_action.execute();
                }
            }
            GameState::RoundEnd => todo!(),
        }
    }
}

fn calculate_actions_from_tile_entities<'a>(game_state: &GameState, robots: &'a mut [Robot], board: &Board)-> Vec<ScheduledAction<'a>>{
    let active_tile_entities = board.tile_eintities.get(game_state).unwrap_or(&vec![]).clone();
    let mut robot_map = robots.iter_mut().map(|robot| (robot.position,robot)).collect::<HashMap<_,_>>();
    let mut scheduled_actions: Vec<ScheduledAction<'a>> = vec![];
    for tile_command in active_tile_entities {
        let action = match tile_command {
            TileCommand::ForRobot(pos, cmd) => {
                let robot = robot_map.remove(&pos);
                robot.map(|robot| ScheduledAction::from(ScheduledRobotCommand(robot,cmd)))
            },
            TileCommand::Laser(pos, dir, intensity) => {
                let laser_positions = board.all_pos_inbounds_in_direction(pos, dir);
                let mut laser_action = None;
                for pos in laser_positions {
                    match robot_map.remove(&pos) {
                        Some(robot) => {
                            laser_action = Some(ScheduledAction(robot,vec![RobotAction::TakeDamage(intensity)]));
                            break;
                        }
                        None if board.direction_blocked(pos, dir) => break,
                        None => continue
                    }
                }
                laser_action
            },
        };
        if let Some(a) = action { scheduled_actions.push(a) }
    }
    todo!()
}


fn resolve_movement<'a>(
    robot_actions: Vec<ScheduledAction<'a>>,
    board: &Board,
) -> Vec<ScheduledAction<'a>> {
    todo!()
}
