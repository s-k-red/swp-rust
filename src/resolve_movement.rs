use itertools::Itertools;

use crate::{
    commands::RobotActionInPlace,
    components::{Board, Robot},
    datatypes::{Direction, Position},
    game_states::GameState,
    scheduled_commands::{ScheduledActions, ScheduledMove},
};

pub fn resolve_factory_movement<'a>(
    mut robot_moves: Vec<ScheduledMove<'a>>,
    board: &'a Board,
    game_state: &GameState,
) -> Vec<ScheduledActions<'a>> {
    (robot_moves, _) = cancel_walls(robot_moves, board);
    (robot_moves, _) = cancel_with_check(robot_moves, board, test_oppoosing);
    let (mut robot_moves, mut any_cancelled) =
        cancel_with_check(robot_moves, board, test_moving_to_same);
    while any_cancelled {
        (robot_moves, any_cancelled) = cancel_with_check(robot_moves, board, test_moving_to_same);
    }
    calculate_on_entry(robot_moves, board, game_state)
}

fn cancel_with_check<'a>(
    robot_moves: Vec<ScheduledMove<'a>>,
    board: &'a Board,
    collision_test: fn((Position, Position), (Position, Position), &Board) -> bool,
) -> (Vec<ScheduledMove<'a>>, bool) {
    let mut any_cancelled = false;
    let mut robot_moves = robot_moves
        .into_iter()
        .map(|scheduled_move| (false, scheduled_move))
        .collect::<Vec<_>>();

    for (a, b) in (0..robot_moves.len()).tuple_combinations() {
        let tuple_represents_collision = {
            let action_a = &robot_moves.get(a).unwrap().1;
            let action_b = &robot_moves.get(b).unwrap().1;
            collision_test(
                (action_a.robot.position, action_a.simulate()),
                (action_b.robot.position, action_b.simulate()),
                board,
            )
        };
        if tuple_represents_collision {
            any_cancelled = true;
            robot_moves.get_mut(a).unwrap().0 = true;
            robot_moves.get_mut(b).unwrap().0 = true;
        }
    }
    (
        robot_moves
            .into_iter()
            .map(|(cancelled, mut scheduled_move)| {
                match cancelled {
                    true => {
                        scheduled_move.mov = None;
                    }
                    false => (),
                }
                scheduled_move
            })
            .collect::<Vec<_>>(),
        any_cancelled,
    )
}

fn test_oppoosing(
    pair1: (Position, Position),
    pair2: (Position, Position),
    _board: &Board,
) -> bool {
    pair1.0 == pair2.1 && pair1.1 == pair2.0
}
fn test_moving_to_same(
    pair1: (Position, Position),
    pair2: (Position, Position),
    board: &Board,
) -> bool {
    pair1.1 == pair2.1 && board.is_inbounds(pair1.1) && (pair1.0 != pair2.0 && pair1.0 != pair1.1)
}

pub fn resolve_card_movement<'a>(
    mut robot_move: ScheduledMove<'a>, //will only contain one actual move
    mut robots: Vec<&'a mut Robot>,
    board: &'a Board,
    game_state: &GameState,
) -> Vec<ScheduledActions<'a>> {
    let mut res = Vec::new();
    loop {
        if board.direct_way_blocked(robot_move.robot.position, robot_move.simulate()) {
            return vec![];
        }

        let next_pos = robot_move.simulate();
        match robots
            .iter()
            .filter(|robot| robot.position == next_pos)
            .count()
        {
            c if 1 < c => return vec![],
            c if 1 == c => {
                let mov = robot_move.mov;
                res.push(robot_move);
                robots.sort_unstable_by_key(|robot| (robot.position != next_pos));
                robot_move = ScheduledMove {
                    mov,
                    robot: robots.swap_remove(0),
                }
            }
            _ => {
                res.push(robot_move);
                break;
            }
        }
    }
    calculate_on_entry(res, board, game_state)
}

pub fn calculate_on_entry<'a>(
    robot_moves: Vec<ScheduledMove<'a>>,
    board: &'a Board,
    game_state: &GameState,
) -> Vec<ScheduledActions<'a>> {
    robot_moves
        .into_iter()
        .map(|robot_move| match robot_move.mov {
            Some(dir) => {
                let actions = calculate_action_for_entry(game_state, robot_move.robot, dir, board)
                    .into_iter()
                    .collect();
                ScheduledActions {
                    robot: robot_move.robot,
                    mov: Some(dir),
                    actions,
                }
            }
            None => ScheduledActions::new(robot_move.robot),
        })
        .collect()
}

pub fn cancel_all(robot_moves: Vec<ScheduledMove>) -> Vec<ScheduledActions> {
    robot_moves
        .into_iter()
        .map(|mov| ScheduledActions::new(mov.robot))
        .collect()
}

pub fn cancel_walls<'a>(
    robot_moves: Vec<ScheduledMove<'a>>,
    board: &'a Board,
) -> (Vec<ScheduledMove<'a>>, bool) {
    let mut any_cancelled = false;
    (
        robot_moves
            .into_iter()
            .map(|mut robot_move| {
                if let Some(mov) = robot_move.mov {
                    if board.direct_way_blocked(robot_move.robot.position, robot_move.simulate()) {
                        robot_move.mov = None;
                        any_cancelled = true;
                    } else {
                        robot_move.mov = Some(mov);
                    }
                }
                robot_move
            })
            .collect::<Vec<ScheduledMove<'a>>>(),
        any_cancelled,
    )
}

fn calculate_action_for_entry(
    game_state: &GameState,
    robot: &Robot,
    move_direction: Direction,
    board: &Board,
) -> Vec<RobotActionInPlace> {
    let destination = robot.position + Position::from(move_direction);
    if !board.is_inbounds(destination) {
        return vec![RobotActionInPlace::Destroy];
    }
    board
        .on_entry_tile_eintities
        .get(game_state)
        .and_then(|hash_map| hash_map.get(&destination).cloned())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|entry| {
            match entry.activation_direction {
                Some(dir) => {
                    if dir == move_direction {
                        return Some(entry.action);
                    }
                }
                None => return Some(entry.action),
            }
            None
        })
        .collect::<Vec<_>>()
}
