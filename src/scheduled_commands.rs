use crate::{
    commands::{RobotAction, RobotActionInPlace, RobotCommand},
    components::Robot,
    datatypes::{Direction, Position},
};
#[derive(Debug)]
pub struct ScheduledActions<'a> {
    pub robot: &'a mut Robot,
    pub mov: Option<Direction>,
    pub actions: Vec<RobotActionInPlace>,
}
#[derive(Debug)]
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
