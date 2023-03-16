#![allow(dead_code)]
#![allow(unused_variables)]

pub(crate) mod commands {

    use crate::components::components::{GameStore, Robot};
    use crate::datatypes::datatypes::{Direction, Position};

    #[derive(Debug, Clone)]
    pub enum RobotCommand {
        Turn(Direction),
        DeclareAbsoluteMove(Direction),
        DeclareRelativeMove(Direction),
        Destroy,
        LeaveSafetyCopy,
    }

    #[derive(Debug, Clone)]
    pub enum TileCommand {
        ForRobot(Position, RobotCommand),
        Laser(Position, Direction, u32),
    }

    pub trait RobotCommandAction {
        fn action(&self, robot: &mut Robot);
    }
    
    pub trait TileCommandAction {
        fn action(&self, game_store: &mut GameStore);
    }

    impl RobotCommandAction for RobotCommand {
        fn action(&self, robot: &mut Robot) {
            match self {
                RobotCommand::Turn(direction) => {
                    robot.facing_direction = robot.facing_direction.turn(*direction)
                }
                RobotCommand::DeclareAbsoluteMove(direction) => {
                    robot.declared_move = *direction
                }
                RobotCommand::DeclareRelativeMove(direction) => {
                    robot.declared_move = robot.facing_direction.turn(*direction)
                }
                RobotCommand::Destroy => robot.alive = false,
                RobotCommand::LeaveSafetyCopy => robot.safety_copy_position = robot.position,
            }
        }
    }

    impl TileCommandAction for TileCommand {
        fn action(&self, game_store: &mut GameStore) {
            match self {
                TileCommand::ForRobot(pos, robot_command_type) => {
                    if let Some(robot) = game_store.get_robot(*pos) {
                        robot_command_type.action(robot);
                    }
                }
                TileCommand::Laser(pos, dir, intensity) => {
                    game_store.laser(*pos, *dir, *intensity);
                }
            }
        }
    }
}
