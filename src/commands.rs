#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

use crate::components::Robot;
use crate::datatypes::{Direction, Position};

#[derive(Debug, Clone)]
pub enum RobotCommand {
    Absolute(RobotAction),
    DeclareRelativeMove(Direction),
}

#[derive(Debug, Clone)]
pub enum RobotAction {
    Move(RobotActionMove),
    InPlace(RobotActionInPlace),
}

#[derive(Debug, Clone)]
pub enum RobotActionMove {
    Move(Direction),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RobotActionInPlace {
    Turn(Direction),
    Repair(usize),
    TakeDamage(usize),
    Destroy,
    LeaveSafetyCopy,
}
pub struct ScheduledActions<'a> {
    pub robot: &'a mut Robot,
    pub mov: Option<RobotActionMove>,
    actions: Vec<RobotActionInPlace>,
}
pub struct ScheduledMoves<'a> {
    pub robot: &'a mut Robot,
    pub own_move: Option<RobotActionMove>,
}
pub struct ChainedMove {
    pub dependent_on: Vec<String>,
    pub mov: RobotActionMove,
}

impl<'a> ScheduledActions<'a> {
    pub fn push<T: Into<RobotAction>>(&mut self, action: T) {
        match action.into() {
            RobotAction::Move(m) => {
                self.mov = Some(m);
            }
            RobotAction::InPlace(in_place) => {
                self.actions.push(in_place);
            }
        }
    }
    pub fn push_and_convert(&mut self, cmd: RobotCommand) {
        let action = match cmd {
            RobotCommand::Absolute(robot_action) => robot_action,
            RobotCommand::DeclareRelativeMove(dir) => {
                RobotAction::from(RobotActionMove::Move(dir * self.robot.facing_direction))
            }
        };
        self.push(action)
    }
    pub fn new(robot: &'a mut Robot) -> Self {
        ScheduledActions {
            robot,
            actions: vec![],
            mov: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TileCommand {
    FromRobotCommand(Position, RobotCommand),
    Laser(Position, Direction, usize),
}

impl RobotActionInPlace {
    fn action(&self, robot: &mut Robot) {
        match self {
            RobotActionInPlace::Turn(direction) => {
                robot.facing_direction = robot.facing_direction.turn(*direction)
            }
            RobotActionInPlace::TakeDamage(amount) => robot.damage(*amount),
            RobotActionInPlace::Repair(amount) => robot.repair(*amount),
            RobotActionInPlace::Destroy => robot.alive = false,
            RobotActionInPlace::LeaveSafetyCopy => robot.safety_copy_position = robot.position,
        }
    }
}

impl RobotActionMove {
    fn action(&self, robot: &mut Robot) {
        robot.position = self.simulate(robot);
    }
    fn simulate(&self, robot: &Robot) -> Position {
        match self {
            RobotActionMove::Move(dir) => robot.position + Position::from(*dir),
        }
    }
}
impl From<RobotActionMove> for RobotAction {
    fn from(value: RobotActionMove) -> Self {
        RobotAction::Move(value)
    }
}
impl From<RobotActionInPlace> for RobotAction {
    fn from(value: RobotActionInPlace) -> Self {
        RobotAction::InPlace(value)
    }
}

pub fn execute_non_moves(actions: ScheduledActions) -> ScheduledMoves {
    for action in &actions.actions {
        action.action(actions.robot)
    }
    ScheduledMoves {
        robot: actions.robot,
        own_move: actions.mov,
    }
}

pub fn execute(actions: ScheduledActions) {
    for action in &actions.actions {
        action.action(actions.robot)
    }
    if let Some(unwrap_mov) = actions.mov {
        unwrap_mov.action(actions.robot)
    }
}