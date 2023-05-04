#![allow(dead_code)]
#![allow(unused_variables)]

use std::iter;

use crate::{
    commands::{RobotActionInPlace, ScheduledActions, ScheduledMoves},
    components::Board,
    datatypes::Position,
};
use itertools::Itertools;

pub fn resolve_movement<'a>(
    robot_actions: Vec<ScheduledMoves<'a>>,
    board: &'a Board,
) -> Vec<ScheduledActions<'a>> {
    todo!()
}


