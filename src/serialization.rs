use crate::{
    commands::{
        IndirectTileEntity, OnEntryTileEntity, RobotAction, RobotActionInPlace, TileEntity,
    },
    datatypes::{Direction, Position},
    game_states::GameState,
};

pub struct TileSerialize {
    position: Position,
    direction: Direction,
    entity: TileEntitySerialize,
}

pub enum TileEntitySerialize {
    Direct(Vec<GameState>, RobotActionSerialize),
    Indirect(Vec<GameState>, IndirectTileEntitySerialize),
    OnEntry(Vec<GameState>, OnEntryTileEntitySerialize),
    Inbounds,
    Wall,
}

pub enum RobotActionSerialize {
    Move,
    InPlace(RobotActionInPlaceSerialize),
}

pub enum RobotActionInPlaceSerialize {
    Turn,
    Repair(i8),
    TakeDamage(i8),
    Destroy,
    LeaveSafetyCopy,
    ReachCheckpointAndLeaveSafetyCopy(usize),
}

pub enum IndirectTileEntitySerialize {
    Laser(i8),
}
pub enum OnEntryTileEntitySerialize {
    WithDirection(RobotActionInPlaceSerialize, Direction),
    withoutDirection(RobotActionInPlaceSerialize),
}

impl From<TileSerialize> for TileEntity {
    fn from(value: TileSerialize) -> Self {
        match value.entity {
            TileEntitySerialize::Direct(game_states, action) => TileEntity::Direct(
                game_states,
                value.position,
                match action {
                    RobotActionSerialize::InPlace(action) => RobotAction::InPlace(
                        convert_robot_action_in_place_serialize(action, Some(value.direction)),
                    ),
                    RobotActionSerialize::Move => RobotAction::Move(value.direction),
                },
            ),
            TileEntitySerialize::Indirect(game_states, action) => TileEntity::Indirect(
                game_states,
                match action {
                    IndirectTileEntitySerialize::Laser(intensity) => {
                        IndirectTileEntity::Laser(value.position, value.direction, intensity)
                    }
                },
            ),
            TileEntitySerialize::OnEntry(game_states, action) => TileEntity::OnEntry(
                game_states,
                value.position,
                OnEntryTileEntity {
                    action: match action {
                        OnEntryTileEntitySerialize::WithDirection(action_in_place, direction) => {
                            convert_robot_action_in_place_serialize(
                                action_in_place,
                                Some(direction * value.direction),
                            )
                        }

                        OnEntryTileEntitySerialize::withoutDirection(action_in_place) => {
                            convert_robot_action_in_place_serialize(action_in_place, None)
                        }
                    },
                    activation_direction: match action {
                        OnEntryTileEntitySerialize::WithDirection(action_in_place, direction) => {
                            Some(direction * value.direction)
                        }

                        OnEntryTileEntitySerialize::withoutDirection(_) => None,
                    },
                },
            ),
            TileEntitySerialize::Inbounds => TileEntity::Inbounds(value.position),
            TileEntitySerialize::Wall => TileEntity::Wall(value.position, value.direction),
        }
    }
}

fn convert_robot_action_in_place_serialize(
    action_serialize: RobotActionInPlaceSerialize,
    direction: Option<Direction>,
) -> RobotActionInPlace {
    let actual_direction = match direction {
        Some(dir) => dir,
        None => Direction::default(),
    };

    match action_serialize {
        RobotActionInPlaceSerialize::Turn => RobotActionInPlace::Turn(actual_direction),
        RobotActionInPlaceSerialize::Repair(amount) => RobotActionInPlace::Repair(amount),
        RobotActionInPlaceSerialize::TakeDamage(amount) => RobotActionInPlace::TakeDamage(amount),
        RobotActionInPlaceSerialize::Destroy => RobotActionInPlace::Destroy,
        RobotActionInPlaceSerialize::LeaveSafetyCopy => RobotActionInPlace::LeaveSafetyCopy,
        RobotActionInPlaceSerialize::ReachCheckpointAndLeaveSafetyCopy(order) => {
            RobotActionInPlace::ReachCheckpointAndLeaveSafetyCopy(order)
        }
    }
}
