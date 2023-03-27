#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    commands::{RobotActionInPlace, ScheduledActions, TransitionLocks},
    components::Board,
    datatypes::Position,
};
use itertools::Itertools;

pub fn resolve_movement<'a>(
    robot_actions: Vec<ScheduledActions<'a>>,
    board: &'a Board,
    locked_transitions: TransitionLocks,
) -> Vec<ScheduledActions<'a>> {
    let (robot_actions, locked_transitions) =
        resolve_walls(robot_actions, board, locked_transitions);
    let (robot_actions, locked_transitions) =
        cancel_with_check(robot_actions, board, locked_transitions, test_oppoosing);
    let (robot_actions, locked_transitions) = cancel_with_check(
        robot_actions,
        board,
        locked_transitions,
        test_moving_to_same,
    );
    let robot_actions = propagate_moves(robot_actions, board, &locked_transitions);
    let (robot_actions, locked_transitions) =
        resolve_walls(robot_actions, board, locked_transitions);
    match !test_board_consistent(&robot_actions, board) {
        true => resolve_movement(robot_actions, board, locked_transitions),
        false => destroy_out_of_bounds(robot_actions, board),
    }
}

fn resolve_walls<'a>(
    robot_actions: Vec<ScheduledActions<'a>>,
    board: &Board,
    mut locked_transitions: TransitionLocks,
) -> (Vec<ScheduledActions<'a>>, TransitionLocks) {
    let mut res = vec![];
    for mut actions in robot_actions {
        if let Some(pos) = actions.simulate_movement() {
            if locked_transitions.check_and_reset(&mut actions) {
                locked_transitions.propagate_block(actions.robot.position, pos);
                actions.mov = None;
            }
        }
        res.push(actions)
    }
    (res, locked_transitions)
}

fn cancel_with_check<'a>(
    robot_actions: Vec<ScheduledActions<'a>>,
    board: &'a Board,
    mut locked_transitions: TransitionLocks,
    collision_test: fn((Position, Position), (Position, Position), &Board) -> bool,
) -> (Vec<ScheduledActions<'a>>, TransitionLocks) {
    let mut action_position_tuples = to_action_position_tuples(robot_actions);
    for (a, b) in (0..action_position_tuples.len()).tuple_combinations() {
        let tuple_represents_collision = {
            let action_pos_tuple_a = action_position_tuples.get(a).unwrap();
            let action_pos_tuple_b = action_position_tuples.get(b).unwrap();
            match action_pos_tuple_a.0.scheduled_dead_or_dead()
                || action_pos_tuple_b.0.scheduled_dead_or_dead()
                || action_pos_tuple_a.1.is_none()
                || action_pos_tuple_b.1.is_none()
            {
                true => continue,
                false => collision_test(
                    (
                        action_pos_tuple_a.0.robot.position,
                        action_pos_tuple_a.1.unwrap(),
                    ),
                    (
                        action_pos_tuple_b.0.robot.position,
                        action_pos_tuple_b.1.unwrap(),
                    ),
                    board,
                ),
            }
        };
        if tuple_represents_collision {
            let action_pos_tuple_a = action_position_tuples.get(a).unwrap();
            let action_pos_tuple_b = action_position_tuples.get(b).unwrap();
            locked_transitions.propagate_block(
                action_pos_tuple_a.0.robot.position,
                action_pos_tuple_a.1.unwrap(),
            );
            locked_transitions.propagate_block(
                action_pos_tuple_b.0.robot.position,
                action_pos_tuple_b.1.unwrap(),
            );
            action_position_tuples.get_mut(a).unwrap().0.mov = None;
            action_position_tuples.get_mut(b).unwrap().0.mov = None;
        }
    }
    (
        from_action_position_tuples(action_position_tuples),
        locked_transitions,
    )
}

fn test_oppoosing(pair1: (Position, Position), pair2: (Position, Position), board: &Board) -> bool {
    pair1.0 == pair2.1 && pair1.1 == pair2.0
}
fn test_moving_to_same(
    pair1: (Position, Position),
    pair2: (Position, Position),
    board: &Board,
) -> bool {
    pair1.1 == pair2.1 && board.is_inbounds(pair1.1)
}

fn to_action_position_tuples(
    robot_actions: Vec<ScheduledActions>,
) -> Vec<(ScheduledActions, Option<Position>)> {
    robot_actions
        .into_iter()
        .map(|sch| {
            let pos_opt = sch.simulate_movement();
            (sch, pos_opt)
        })
        .collect::<Vec<_>>()
}
fn from_action_position_tuples(
    action_position_tuples: Vec<(ScheduledActions, Option<Position>)>,
) -> Vec<ScheduledActions> {
    action_position_tuples
        .into_iter()
        .map(|(actions, _)| actions)
        .collect::<Vec<_>>()
}

fn propagate_moves<'a>(
    robot_actions: Vec<ScheduledActions<'a>>,
    board: &Board,
    locked_transitions: &TransitionLocks,
) -> Vec<ScheduledActions<'a>> {
    let mut action_position_tuples = to_action_position_tuples(robot_actions);
    for (mut a, mut b) in (0..action_position_tuples.len()).tuple_combinations() {
        let can_propagate_move = {
            let action_pos_tuple_a = action_position_tuples.get(a).unwrap();
            let action_pos_tuple_b = action_position_tuples.get(b).unwrap();
            match action_pos_tuple_a.0.scheduled_dead_or_dead()
                || action_pos_tuple_b.0.scheduled_dead_or_dead()
            {
                true => continue,
                false => {
                    if action_pos_tuple_a.1.is_some() && action_pos_tuple_b.1.is_none() {
                        true
                    } else {
                        std::mem::swap(&mut a, &mut b);
                        action_pos_tuple_a.1.is_some() && action_pos_tuple_b.1.is_none()
                    }
                }
            }
        };
        if can_propagate_move {
            let propagated_move_action = action_position_tuples
                .get(a)
                .unwrap()
                .0
                .mov
                .clone()
                .unwrap()
                .0;
            action_position_tuples.get_mut(b).unwrap().0.mov =
                Some((propagated_move_action, false));
        }
    }
    from_action_position_tuples(action_position_tuples)
}

fn test_board_consistent(robot_actions: &[ScheduledActions], board: &Board) -> bool {
    robot_actions
        .iter()
        .map(|actions| {
            actions
                .simulate_movement()
                .unwrap_or(actions.robot.position)
        })
        .filter(|pos| board.is_inbounds(*pos))
        .all_unique()
}

fn destroy_out_of_bounds<'a>(
    robot_actions: Vec<ScheduledActions<'a>>,
    board: &Board,
) -> Vec<ScheduledActions<'a>> {
    let mut res = vec![];
    for mut actions in robot_actions {
        if let Some(pos) = actions.simulate_movement() {
            if !board.is_inbounds(pos) {
                actions.insert(RobotActionInPlace::Destroy);
            }
        }
        res.push(actions)
    }
    res
}
