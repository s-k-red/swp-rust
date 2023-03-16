#![allow(dead_code)]
#![allow(unused_variables)]

pub(crate) mod components {
    use std::collections::{HashMap, HashSet};
    use std::hash::Hash;

    use crate::commands::commands::{RobotCommand, TileCommand};
    use crate::datatypes::datatypes::{Direction, Position};
    use crate::game_states::gamestates::GameState;

    #[derive(Debug)]
    pub struct Robot {
        pub position: Position,
        pub declared_move: Direction,
        pub facing_direction: Direction,
        pub user_name: String,
        pub safety_copy_position: Position,
        pub alive: bool,
        pub played_cards: Vec<Card>,
    }

    #[derive(Debug)]
    pub struct GameStore {
        pub robots: Vec<Robot>,
        walls: HashSet<Wall>,
        pos_inbounds: HashSet<Position>,
        pub tile_eintities: HashMap<GameState, Vec<TileCommand>>,
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Wall(Position, Position);

    impl GameStore {
        pub fn direction_blocked(&self, pos: Position, dir: Direction) -> bool {
            self.walls.contains(&Wall(pos, pos + dir.to_position()))
                || self.walls.contains(&Wall(pos + dir.to_position(), pos))
        }

        pub fn get_robot(&mut self, pos: Position) -> Option<&mut Robot> {
            self.robots
                .iter_mut()
                .filter(|robot| robot.alive)
                .find(|robot| robot.position == pos)
        }

        pub fn all_pos_inbounds_in_direction(
            &self,
            pos: Position,
            dir: Direction,
        ) -> Vec<Position> {
            self.pos_inbounds
                .iter()
                .filter(|filter_pos| pos.on_axis(dir, **filter_pos))
                .map(|pos| *pos)
                .collect::<Vec<Position>>()
        }

        pub fn laser(&mut self, pos: Position, dir: Direction, intensity: u32) {
            let mut laser_positions = self.all_pos_inbounds_in_direction(pos, dir);
            laser_positions.sort_by_key(|laser_position| pos.distance(*laser_position));

            for pos in laser_positions {
                match self.get_robot(pos) {
                    Some(robot) => {
                        robot.damage(intensity);
                        return;
                    }
                    None => {
                        if self.direction_blocked(pos, dir) {
                            return;
                        }
                    }
                }
            }
        }
    }

    impl Robot {
        pub fn damage(&mut self, amount: u32) {
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
}
