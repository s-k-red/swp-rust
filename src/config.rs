//INPUT NODES: handcards + played cards + my position (x+y) + my direction + my health + extra lives + 6 checkpoint positions (filled with -1s if less are placed)
//              + 4 tile entities/tile so 4 * 12 * 12 * (orientation ordinal + typeid + position) + 4 other robot positions + directions (filled with -1s if less are playing)

use crate::datatypes::Position;

//INPUT NODES: handcards + played cards + my position (x+y) + (my last position erstmal ohne) + my direction + my health + extra lives + next checkpoint position
//              + 4 tile entities/tile so 4 * 12 * 12 * (orientation ordinal + typeid ) + 4 other robot positions + directions (filled with -1s if less are playing)
pub const INPUT_NODES: usize = 9 + 5 + 2 + 1 + 1 + 1 + 1 * 2 + 4 * 12 * 12 * 2 + 4 * 3; //9 + 5 + 2 + 1 + 1 + 1 + 6 * 2 + 4 * 12 * 12 * 4 + 4 * 3;

//Amount of hand cards
pub const OUTPUT_NODES: usize = 9;
pub const HIDDEN_LAYERS: usize = 10;
pub const MUTATION_RATE: f32 = 0.001; //0.01%
pub const GENERATIONS: usize = 50;
pub const PUPULATION_SIZE: i32 = 1000;
pub const CHECKPOINTS: &[&[Position]] = &[
    &[Position { x: 7, y: 10 }, Position { x: 2, y: 10 }, Position {x: 1, y: 7}, Position {x: 1, y: 2}, Position {x: 3, y: 2}],
    &[Position { x: 7, y: 10 }, Position { x: 2, y: 10 }, Position {x: 1, y: 7}, Position {x: 1, y: 2}, Position {x: 3, y: 2}],
    &[Position { x: 7, y: 4 }, Position { x: 2, y: 8 }],
    &[Position { x: 1, y: 0 }, Position { x: 7, y: 8 }, Position { x: 1, y: 7 }, Position { x: 7, y: 9 }, Position { x: 10, y: 8 }],
];
pub const ROUND_THRESHOLD: usize = 30;
pub const HIDDEN_NODES: usize = INPUT_NODES / 3;