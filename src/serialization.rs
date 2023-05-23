use crate::{
    commands::{
        IndirectTileEntity, OnEntryTileEntity, RobotAction, RobotActionInPlace, TileEntity,
    },
    datatypes::{Direction, Position},
    game_states::GameState,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SourceLayer {
    pub data: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct SourceMap {
    pub layers: Vec<SourceLayer>,
}

#[derive(Serialize, Deserialize)]
pub struct Tileset {
    pub tiles: Vec<TilesetTile>,
}

#[derive(Serialize, Deserialize)]
pub struct TilesetTile {
    pub properties: Vec<TilesetTileProperty>,
    pub id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct TilesetTileProperty {
    pub value: Vec<TileEntitySerialize>,
}

pub struct TileSerialize {
    pub position: Position,
    pub direction: Direction,
    pub entity: TileEntitySerialize,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TileEntitySerialize {
    Direct(Vec<GameState>, RobotActionSerialize),
    Indirect(Vec<GameState>, IndirectTileEntitySerialize),
    OnEntry(Vec<GameState>, OnEntryTileEntitySerialize),
    Inbounds,
    Wall,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RobotActionSerialize {
    Move,
    InPlace(RobotActionInPlaceSerialize),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RobotActionInPlaceSerialize {
    Turn,
    Repair(i8),
    TakeDamage(i8),
    Destroy,
    LeaveSafetyCopy,
    ReachCheckpointAndLeaveSafetyCopy(usize),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum IndirectTileEntitySerialize {
    Laser(i8),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum OnEntryTileEntitySerialize {
    WithDirection(RobotActionInPlaceSerialize, Direction),
    WithoutDirection(RobotActionInPlaceSerialize),
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
            TileEntitySerialize::OnEntry(game_states, action) => {
                let activation_direction = match action {
                    OnEntryTileEntitySerialize::WithDirection(_, direction) => {
                        Some(direction * value.direction)
                    }

                    OnEntryTileEntitySerialize::WithoutDirection(_) => None,
                };
                TileEntity::OnEntry(
                    game_states,
                    value.position,
                    OnEntryTileEntity {
                        action: match action {
                            OnEntryTileEntitySerialize::WithDirection(
                                action_in_place,
                                direction,
                            ) => convert_robot_action_in_place_serialize(
                                action_in_place,
                                Some(direction * value.direction),
                            ),

                            OnEntryTileEntitySerialize::WithoutDirection(action_in_place) => {
                                convert_robot_action_in_place_serialize(action_in_place, None)
                            }
                        },
                        activation_direction,
                    },
                )
            }
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
