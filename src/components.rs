#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use itertools::Itertools;

use crate::commands::{RobotCommand, TileCommand};
use crate::datatypes::{Direction, Position};
use crate::game_states::GameState;

#[derive(Debug)]
pub struct Robot {
    pub user_name: String,
    pub position: Position,
    pub facing_direction: Direction,
    pub safety_copy_position: Position,
    pub alive: bool,
    pub locked_card_slots: Vec<bool>,
}

#[derive(Debug)]
pub struct Player {
    pub user_name: String,
    pub cards_in_hand: Vec<Card>,
    pub cards_played: Vec<Card>,
}
#[derive(Debug)]
pub struct GameStore {
    pub robots: Vec<Robot>,
    pub player: Vec<Player>,
    pub board: Board,
    pub robot_settings: RobotSettings,
}
#[derive(Debug)]
pub struct RobotSettings {
    pub max_hp: usize,
    pub card_slots: usize,
}

#[derive(Debug)]
pub struct Board {
    pub walls: HashSet<Wall>,
    pos_inbounds: HashSet<Position>,
    pub tile_eintities: HashMap<GameState, Vec<TileCommand>>,
}

#[derive(Debug,Clone)]
pub struct Wall(pub Position,pub Position);
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
}

impl Robot {
    pub fn damage(&mut self, amount: usize) {
        todo!()
    }

    pub(crate) fn repair(&self, amount: usize) {
        todo!()
    }
}

#[derive(Debug)]
pub struct Card {
    pub id: u32,
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
