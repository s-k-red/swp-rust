use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use itertools::Itertools;

use crate::commands::{
    IndirectTileEntity, OnEntryTileEntity, RobotAction, RobotCommand, TileEntity,
};
use crate::datatypes::{Direction, Position, ALL_DIRECTIONS};
use crate::game_states::GameState;

const STARTING_LIVES: usize = 3;
const MAX_HP: i8 = 10;
const HAND_SIZE: i8 = 5;

#[derive(Debug, Clone)]
pub struct Robot {
    pub user_name: String,
    pub position: Position,
    pub facing_direction: Direction,
    pub safety_copy_position: Position,
    pub safety_copy_number: usize,
    pub safety_copy_amount: usize,
    pub greatest_checkpoint_reached: usize,
    pub alive: bool,
    pub hp: i8,
    //pub locked_card_slots: Vec<bool>, not needed in this abstraction
}

#[derive(Debug, Clone)]
pub struct Player {
    pub user_name: String,
    pub cards_in_hand: Vec<Card>,
    pub cards_played: Vec<Card>,
}

#[derive(Debug, Clone, Builder)]
pub struct GameStore {
    pub robots: Vec<Robot>,
    pub players: Vec<Player>,
    pub board: Board,
    pub card_deck: Vec<Card>,
    pub winners: Vec<String>,
    //    pub robot_settings: RobotSettings,
}

//#[derive(Debug)]
//pub struct RobotSettings {
//    pub max_hp: usize,
//    pub card_slots: usize,
//}

#[derive(Debug, Clone)]
pub struct Board {
    pub walls: HashSet<Wall>,
    pos_inbounds: HashSet<Position>,
    pub direct_tile_eintities: HashMap<GameState, HashMap<Position, Vec<RobotAction>>>,
    pub indirect_tile_eintities: HashMap<GameState, Vec<IndirectTileEntity>>,
    pub on_entry_tile_eintities: HashMap<GameState, HashMap<Position, Vec<OnEntryTileEntity>>>,
}

#[derive(Debug, Clone)]
pub struct Wall(pub Position, pub Position);
impl PartialEq for Wall {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for Wall {}

impl Hash for Wall {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0 + self.1).hash(state)
    }
}
impl Board {
    pub fn direct_way_blocked(&self, pos1: Position, pos2: Position) -> bool {
        self.walls.contains(&Wall(pos1, pos2))
    }

    pub fn all_pos_inbounds_in_direction_until_blocked(
        &self,
        pos: Position,
        dir: Direction,
    ) -> Vec<Position> {
        let mut res = self
            .pos_inbounds
            .iter()
            .filter(|filter_pos| pos.on_axis(dir, **filter_pos))
            .copied::<Position>()
            .collect::<Vec<Position>>();
        res.sort_by_key(|res_position| pos.distance(*res_position));
        res.insert(0, pos);
        res.iter()
            .tuple_windows()
            .take_while(|(&pos1, &pos2)| !self.direct_way_blocked(pos1, pos2))
            .map(|(_, pos)| *pos)
            .collect::<Vec<_>>()
    }
    pub fn is_inbounds(&self, pos: Position) -> bool {
        self.pos_inbounds.contains(&pos)
    }

    pub fn new(board: Vec<TileEntity>) -> Self {
        let mut walls: HashSet<Wall> = HashSet::default();
        let mut pos_inbounds: HashSet<Position> = HashSet::default();
        let mut direct_tile_eintities: HashMap<GameState, HashMap<Position, Vec<RobotAction>>> =
            HashMap::default();
        let mut indirect_tile_eintities: HashMap<GameState, Vec<IndirectTileEntity>> =
            HashMap::default();
        let mut on_entry_tile_eintities: HashMap<
            GameState,
            HashMap<Position, Vec<OnEntryTileEntity>>,
        > = HashMap::default();

        for tile_entity in board {
            match tile_entity {
                TileEntity::Direct(game_states, pos, action) => {
                    direct_tile_eintities =
                        insert_help(game_states, pos, action, direct_tile_eintities);
                    pos_inbounds.insert(pos);
                }
                TileEntity::Indirect(game_states, entity) => {
                    for game_state in game_states {
                        match indirect_tile_eintities.get_mut(&game_state) {
                            Some(vec) => {
                                vec.push(entity.clone());
                            }
                            None => {
                                indirect_tile_eintities.insert(game_state, vec![entity.clone()]);
                            }
                        }
                    }

                    pos_inbounds.insert(match entity {
                        IndirectTileEntity::Laser(pos, _, _) => pos,
                    });
                }
                TileEntity::OnEntry(game_states, pos, entity) => {
                    on_entry_tile_eintities =
                        insert_help(game_states, pos, entity, on_entry_tile_eintities);
                    pos_inbounds.insert(pos);
                }
                TileEntity::Empty(pos) => {
                    pos_inbounds.insert(pos);
                }
                TileEntity::Wall(pos, dir) => {
                    walls.insert(Wall(pos, pos + dir.into()));
                    pos_inbounds.insert(pos);
                }
            }
        }

        Board {
            walls,
            pos_inbounds,
            direct_tile_eintities,
            indirect_tile_eintities,
            on_entry_tile_eintities,
        }
    }
}

fn insert_help<T: Clone>(
    game_states: Vec<GameState>,
    pos: Position,
    entity: T,
    mut big_map: HashMap<GameState, HashMap<Position, Vec<T>>>,
) -> HashMap<GameState, HashMap<Position, Vec<T>>> {
    for game_state in game_states {
        match big_map.get_mut(&game_state) {
            Some(map) => match map.get_mut(&pos) {
                Some(vec) => {
                    vec.push(entity.clone());
                }
                None => {
                    map.insert(pos, vec![entity.clone()]);
                }
            },
            None => {
                let mut new_value = HashMap::new();
                new_value.insert(pos, vec![entity.clone()]);
                big_map.insert(game_state, new_value);
            }
        }
    }
    big_map
}

impl Robot {
    pub fn respawn(&mut self, board: &Board, occupied: &mut Vec<Position>) {
        let mut possible_respawn_pos = vec![self.safety_copy_position];
        let mut visited = vec![];

        while occupied
            .iter()
            .all(|pos| possible_respawn_pos.contains(pos))
        {
            visited.append(&mut possible_respawn_pos);
            possible_respawn_pos = vec![];

            for pos in &visited {
                for dir in ALL_DIRECTIONS {
                    let pos_to_inspect = *pos + dir.into();
                    if board.direct_way_blocked(*pos, pos_to_inspect)
                        || board.is_inbounds(pos_to_inspect)
                        || visited.contains(&pos_to_inspect)
                    {
                        continue;
                    }
                    possible_respawn_pos.push(pos_to_inspect);
                }
            }
        }
        let respawn_pos = possible_respawn_pos
            .iter()
            .find(|pos| !occupied.contains(pos))
            .expect("tried respawning occupied");
        self.position = *respawn_pos;
        self.alive = true;
        self.hp = MAX_HP;
        self.safety_copy_amount -= 1;
        occupied.push(*respawn_pos);
    }

    pub fn new(user_name: String, position: Position) -> Self {
        Robot {
            user_name,
            position,
            facing_direction: Direction::new(0),
            safety_copy_position: position,
            safety_copy_number: 0,
            safety_copy_amount: STARTING_LIVES,
            greatest_checkpoint_reached: 0,
            alive: true,
            hp: MAX_HP,
        }
    }
}

impl Player {
    pub fn new(user_name: String) -> Self {
        Player {
            user_name,
            cards_in_hand: vec![],
            cards_played: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub id: u32,
    pub is_movement: bool,
    pub commands: Vec<RobotCommand>,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Hash for Card {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl Eq for Card {}
