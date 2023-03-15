#![allow(dead_code)]
#![allow(unused_variables)]

pub(crate) mod datatypes {

    use std::{fmt::Display, ops::Add};

    #[derive(Clone, Copy, Debug)]
    pub struct Direction {
        ordinal: i8,
    }

    impl Direction {
        pub fn new(mut ordinal: i8) -> Self {
            ordinal = ordinal % 4;
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
        pub fn to_position(&self) -> Position {
            Position {
                x: (self.ordinal % 2 * (if self.ordinal < 2 { 1 } else { -1 })) as i32,
                y: ((self.ordinal + 1) % 2 * (if self.ordinal < 2 { 1 } else { -1 })) as i32,
            }
        }
    }
    impl Display for Direction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.name())
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Position {
        x: i32,
        y: i32,
    }

    impl Position {
        pub fn on_axis(&self, dir: Direction, pos: Position) -> bool {
            let offset = dir.to_position();
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
}
