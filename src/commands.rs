#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashSet;

use hashbag::HashBag;

use crate::components::{Robot, Wall};
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
    pub mov: Option<(RobotActionMove, bool)>,
    actions: HashBag<RobotActionInPlace>,
}
impl<'a> ScheduledActions<'a> {
    pub fn insert<T: Into<RobotAction>>(&mut self, action: T) {
        match action.into() {
            RobotAction::Move(m) => {
                self.mov = Some((m, true));
            }
            RobotAction::InPlace(in_place) => {
                self.actions.insert(in_place);
            }
        }
    }
    pub fn insert_and_convert(&mut self, cmd: RobotCommand) {
        let action = match cmd {
            RobotCommand::Absolute(robot_action) => robot_action,
            RobotCommand::DeclareRelativeMove(dir) => {
                RobotAction::from(RobotActionMove::Move(dir * self.robot.facing_direction))
            }
        };
        self.insert(action)
    }
    pub fn new(robot: &'a mut Robot) -> Self {
        ScheduledActions {
            robot,
            actions: HashBag::new(),
            mov: None,
        }
    }
    pub fn scheduled_dead_or_dead(&self) -> bool {
        0 < self.actions.contains(&RobotActionInPlace::Destroy) || !self.robot.alive
    }
    pub fn simulate_movement(&self) -> Option<Position> {
        self.mov.as_ref().map(|(m, _)| match m {
            RobotActionMove::Move(dir) => self.robot.position + Position::from(*dir),
        })
    }
}

pub struct TransitionLocks {
    strong: HashSet<Wall>,
    weak: HashSet<Wall>,
}

impl TransitionLocks {
    pub fn propagate_block(&mut self, from: Position, to: Position) {
        self.strong.insert(Wall(from, to));
        self.strong.insert(Wall(from, from - (from - to)));
        self.weak.insert(Wall(to, to + (from - to)));
    }
    pub fn check_and_reset(&self, actions: &mut ScheduledActions) -> bool {
        if let Some(pos) = actions.simulate_movement() {
            if self.strong.contains(&Wall(actions.robot.position, pos))
                || (actions.mov.as_ref().unwrap().1
                    && self.weak.contains(&Wall(actions.robot.position, pos)))
            {
                actions.mov = None;
                return true;
            }
        }
        false
    }
    pub fn new(walls: &HashSet<Wall>) -> Self {
        Self {
            strong: HashSet::clone(walls),
            weak: HashSet::new(),
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
        robot.position = match self {
            RobotActionMove::Move(dir) => robot.position + Position::from(*dir),
        };
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

pub fn execute(actions: ScheduledActions) {
    for action in &actions.actions {
        action.action(actions.robot)
    }
}
