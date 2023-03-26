#![allow(dead_code)]
#![allow(unused_variables)]


use crate::components::{Robot};
use crate::datatypes::{Direction, Position};

#[derive(Debug, Clone)]
pub enum RobotCommand {
    Absolute(RobotAction),
    DeclareRelativeMove(Direction),
}

#[derive(Debug, Clone)]
pub enum RobotAction {
    Turn(Direction),
    Move(Direction),
    LockedInPlace,
    Repair(usize),
    TakeDamage(usize),
    Destroy,
    LeaveSafetyCopy,
    None
}
pub struct ScheduledRobotCommand<'a>(pub &'a mut Robot, pub RobotCommand);
pub struct ScheduledAction<'a>(pub &'a mut Robot, pub Vec<RobotAction>);

impl<'a> From<ScheduledRobotCommand<'a>> for ScheduledAction<'a> {
    fn from(value: ScheduledRobotCommand<'a>) -> Self {
        let action = match value.1 {
            RobotCommand::Absolute(robot_action) => robot_action,
            RobotCommand::DeclareRelativeMove(dir) => {
                RobotAction::Move(dir * value.0.facing_direction)
            }
        };
        ScheduledAction(value.0,vec![action])
    }
}

#[derive(Debug, Clone)]
pub enum TileCommand {
    ForRobot(Position, RobotCommand),
    Laser(Position, Direction, usize),
}
impl RobotAction {
    fn action(&self, robot: &mut Robot) {
        match self {
            RobotAction::Turn(direction) => {
                robot.facing_direction = robot.facing_direction.turn(*direction)
            }
            RobotAction::Move(dir) => robot.position = robot.position + Position::from(*dir),
            RobotAction::LockedInPlace => (),
            RobotAction::TakeDamage(amount) => robot.damage(*amount),
            RobotAction::Repair(amount) => robot.repair(*amount),
            RobotAction::Destroy => robot.alive = false,
            RobotAction::LeaveSafetyCopy => robot.safety_copy_position = robot.position,
            RobotAction::None => (),
        }
    }
}
impl ScheduledAction<'_> {
    pub fn execute(&mut self) {
        for action in &self.1 {
            action.action(self.0)
        }
    }
}