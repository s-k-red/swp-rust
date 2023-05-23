use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use serde::{Serialize, Deserialize};
pub const ALL_DIRECTIONS: [Direction; 4] = [
    Direction { ordinal: 0 },
    Direction { ordinal: 1 },
    Direction { ordinal: 2 },
    Direction { ordinal: 3 },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Direction {
    pub ordinal: i8,
}

impl Direction {
    pub fn new(mut ordinal: i8) -> Self {
        ordinal %= 4;
        Self { ordinal }
    }

    pub fn ordinal(&self) -> i8 {
        self.ordinal
    }
    pub fn turn(&self, direction: Direction) -> Direction {
        Direction::new((self.ordinal + direction.ordinal) % 4)
    }
    pub fn name(&self) -> &str {
        match self.ordinal {
            0 => "North",
            1 => "East",
            2 => "South",
            3 => "West",
            _ => unreachable!(),
        }
    }
}
impl Default for Direction {
    fn default() -> Self {
        Direction { ordinal: 0 }
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn on_axis(&self, dir: Direction, pos: Position) -> bool {
        let offset = Position::from(dir);
        if offset.x == 0 && pos.x == self.x {
            return 0 <= offset.y * (pos.y - self.y);
        }
        if offset.y == 0 && pos.y == self.y {
            return 0 <= offset.x * (pos.x - self.x);
        }
        false
    }
    pub fn distance(&self, pos: Position) -> u32 {
        self.x.abs_diff(pos.x) + self.y.abs_diff(pos.y)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Mul for Direction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.turn(rhs)
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        Position {
            x: (value.ordinal % 2 * (if value.ordinal < 2 { 1 } else { -1 })) as i32,
            y: ((value.ordinal + 1) % 2 * (if value.ordinal < 2 { 1 } else { -1 })) as i32,
        }
    }
}
