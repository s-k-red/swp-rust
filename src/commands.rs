#![allow(dead_code)]
#![allow(unused_variables)]

use crate::components::Robot;
use crate::datatypes::{Direction, Position};

#[derive(Debug, Clone)]
pub enum RobotCommand {
    Absolute(RobotAction),
    DeclareRelativeMove(Direction),
}

#[derive(Debug, Clone)]
pub enum RobotAction {
    Move(Direction),
    InPlace(RobotActionInPlace),
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RobotActionInPlace {
    Turn(Direction),
    Repair(i8),
    TakeDamage(i8),
    Destroy,
    LeaveSafetyCopy,
    ReachCheckpointAndLeaveSafetyCopy(usize),
}

#[derive(Debug, Clone)]
pub enum TileEntity {
    FromRobotCommand(Position, RobotAction),
    Laser(Position, Direction, i8),
}

#[derive(Debug, Clone)]
pub struct OnEntryTileEntity {
    pub position: Position,
    pub action: RobotActionInPlace,
    pub direction: Option<Direction>,
}

pub struct ScheduledActions<'a> {
    pub robot: &'a mut Robot,
    pub mov: Option<Direction>,
    pub actions: Vec<RobotActionInPlace>,
}
pub struct ScheduledMove<'a> {
    pub robot: &'a mut Robot,
    pub mov: Option<Direction>,
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
                RobotAction::Move(dir * self.robot.facing_direction)
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

impl RobotActionInPlace {
    fn action(&self, robot: &mut Robot) {
        match self {
            RobotActionInPlace::Turn(direction) => {
                robot.facing_direction = robot.facing_direction.turn(*direction)
            }
            RobotActionInPlace::TakeDamage(amount) => {
                robot.hp -= amount;
                if robot.hp == 0 {
                    robot.alive = false;
                }
            }
            RobotActionInPlace::Repair(amount) => robot.hp += amount,
            RobotActionInPlace::Destroy => robot.alive = false,
            RobotActionInPlace::LeaveSafetyCopy => robot.safety_copy_position = robot.position,
            RobotActionInPlace::ReachCheckpointAndLeaveSafetyCopy(checkpoint_number) => {
                if *checkpoint_number == robot.greatest_checkpoint_reached + 1 {
                    robot.greatest_checkpoint_reached = *checkpoint_number;
                    robot.safety_copy_position = robot.position;
                }
            }
        }
    }
}
impl From<RobotActionInPlace> for RobotAction {
    fn from(value: RobotActionInPlace) -> Self {
        RobotAction::InPlace(value)
    }
}

impl ScheduledMove<'_> {
    pub fn simulate(&self) -> Position {
        match self.mov {
            Some(unwrap) => self.robot.position + unwrap.into(),
            None => self.robot.position,
        }
    }
}

pub fn execute_non_moves(actions: ScheduledActions) -> ScheduledMove {
    for action in &actions.actions {
        action.action(actions.robot)
    }
    ScheduledMove {
        robot: actions.robot,
        mov: actions.mov,
    }
}

pub fn execute(actions: ScheduledActions) {
    if let Some(unwrap_mov) = actions.mov {
        {
            actions.robot.position = actions.robot.position + unwrap_mov.into();
        }
    }
    for action in &actions.actions {
        action.action(actions.robot)
    }
}
