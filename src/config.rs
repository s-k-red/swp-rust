//INPUT NODES: handcards + played cards + my position (x+y) + my direction + my health + extra lives + 6 checkpoint positions (filled with -1s if less are placed)
//              + 4 tile entities/tile so 4 * 12 * 12 * (orientation ordinal + typeid + position) + 4 other robot positions + directions (filled with -1s if less are playing)

use crate::datatypes::Position;

//INPUT NODES: handcards as bits + played cards as bits + (my last position erstmal ohne) + my direction + my health + extra lives + next checkpoint position relativ to robot
// + 4 tile entities/tile on a 3 tile radius view point around the robot so 4 * 7 * 7 * (orientation ordinal + typeid )
pub const INPUT_NODES: usize = 10*9 + 10*5 + 1 + 1 + 1 + 1 * 2 + 4 * 7 * 7 * 2; //9 + 5 + 2 + 1 + 1 + 1 + 6 * 2 + 4 * 12 * 12 * 4 + 4 * 3;

//Amount of hand cards
pub const OUTPUT_NODES: usize = 9;
pub const HIDDEN_LAYERS: usize = 5;
pub const MUTATION_RATE: f32 = 0.001; //0.1%
pub const GENERATIONS: usize = 10;
pub const PUPULATION_SIZE: i32 = 2000;
pub const CHECKPOINTS: &[&[Position]] = &[
    &[Position { x: 7, y: 10 }, Position { x: 2, y: 10 }],
    &[Position { x: 7, y: 4 }, Position { x: 8, y: 8 }],
    &[Position { x: 1, y: 0 }, Position { x: 3, y: 4 }],
    &[Position { x: 7, y: 4 }, Position { x: 7, y: 10 }]
    // &[Position { x: 7, y: 10 }, Position { x: 2, y: 10 }, Position {x: 1, y: 7}, Position {x: 1, y: 2}, Position {x: 3, y: 2}],
    // // &[Position { x: 7, y: 10 }, Position { x: 2, y: 10 }, Position {x: 1, y: 7}, Position {x: 1, y: 2}, Position {x: 3, y: 2}],
    // &[Position { x: 7, y: 4 }, Position { x: 2, y: 8 }],
    // &[Position { x: 1, y: 0 }, Position { x: 7, y: 8 }, Position { x: 1, y: 7 }, Position { x: 7, y: 9 }, Position { x: 10, y: 8 }],
];
pub const ROUND_THRESHOLD: usize = 30;
pub const HIDDEN_NODES: usize = INPUT_NODES / 2;
