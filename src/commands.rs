use crate::components::{Board, Robot};
use crate::datatypes::{Direction, Position};
use crate::game_states::GameState;
use crate::scheduled_commands::ScheduledActions;
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum RobotCommand {
    Absolute(RobotAction),
    DeclareRelativeMove(Direction),
}

#[derive(Debug, Clone, Serialize)]
pub enum RobotAction {
    Move(Direction),
    InPlace(RobotActionInPlace),
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub enum RobotActionInPlace {
    Turn(Direction),
    Repair(i8),
    TakeDamage(i8),
    Destroy,
    LeaveSafetyCopy,
    ReachCheckpointAndLeaveSafetyCopy(usize),
}

#[derive(Debug, Clone, Serialize)]
pub enum TileEntity {
    Direct(Vec<GameState>, Position, RobotAction),
    Indirect(Vec<GameState>, IndirectTileEntity),
    OnEntry(Vec<GameState>, Position, OnEntryTileEntity),
    Inbounds(Position),
    Wall(Position, Direction),
}

#[derive(Debug, Clone, Serialize)]
pub enum IndirectTileEntity {
    Laser(Position, Direction, i8),
}

#[derive(Debug, Clone, Serialize)]
pub struct OnEntryTileEntity {
    pub action: RobotActionInPlace,
    pub activation_direction: Option<Direction>,
}

impl RobotActionInPlace {
    pub fn action(&self, robot: &mut Robot) {
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

impl IndirectTileEntity {
    pub fn convert<'a>(
        &self,
        board: &Board,
        mut actions: Vec<ScheduledActions<'a>>,
    ) -> Vec<ScheduledActions<'a>> {
        match self {
            IndirectTileEntity::Laser(pos, dir, intensity) => {
                for pos in board.all_pos_inbounds_in_direction_until_blocked(*pos, *dir) {
                    for action in &mut actions {
                        if action.robot.position == pos {
                            action.push(RobotActionInPlace::TakeDamage(*intensity));
                        }
                    }
                }
            }
        }
        actions
    }
}
