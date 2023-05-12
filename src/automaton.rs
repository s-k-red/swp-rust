#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    commands::{execute, execute_non_moves, ScheduledActions},
    components::{Board, Card, Player, Robot},
    game_states::GameState,
    resolve_movement::{resolve_card_movement, resolve_factory_movement},
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
                                    actions.push_and_convert(cmd.clone());
                                }
                                ScheduledActions::new(robot)
                            })
                            .unwrap()];
                        let robot_moves =
                            robot_actions.into_iter().map(execute_non_moves).collect();
                        let robot_actions = resolve_card_movement(robot_moves, board, self);
                        for robot_action in robot_actions {
                            execute(robot_action);
                        }
                    }
                }
            }
            GameState::FactoryState(_, _) => {
                let robot_actions = calculate_actions_from_tile_entities(self, robots, board);
                let robot_moves = robot_actions.into_iter().map(execute_non_moves).collect();
                let robot_actions = resolve_factory_movement(robot_moves, board, self);
                for robot_action in robot_actions {
                    execute(robot_action);
                }
            }
            GameState::RoundEnd => {
                let robot_actions = calculate_actions_from_tile_entities(self, robots, board);
                let robot_moves = robot_actions.into_iter().map(execute_non_moves).collect();
                let robot_actions = resolve_factory_movement(robot_moves, board, self);
                for robot_action in robot_actions {
                    execute(robot_action);
                }
                let mut occupied = robots
                    .iter()
                    .filter(|robot| robot.alive)
                    .map(|robot| robot.position)
                    .collect::<Vec<_>>();
                robots
                    .iter_mut()
                    .filter(|robot| !robot.alive && robot.safety_copy_amount > 0)
                    .for_each(|robot| robot.respawn(board, &mut occupied));
            }
        }
    }
}

fn calculate_actions_from_tile_entities<'a>(
    game_state: &GameState,
    robots: &'a mut [Robot],
    board: &Board,
) -> Vec<ScheduledActions<'a>> {
    let mut actions = robots
        .iter_mut()
        .map(ScheduledActions::new)
        .collect::<Vec<_>>();

    let indirect_entities = board
        .indirect_tile_eintities
        .get(game_state)
        .cloned()
        .unwrap_or_default();

    for indirect_entity in indirect_entities {
        actions = indirect_entity.convert(board, actions);
    }

    for action in &mut actions {
        if let Some(tile_actions) = board
            .direct_tile_eintities
            .get(game_state)
            .and_then(|all_active| all_active.get(&action.robot.position))
        {
            tile_actions
                .iter()
                .for_each(|tile_action| action.push(tile_action.clone()));
        }
    }

    actions
}
