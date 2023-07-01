//INPUT NODES: handcards + played cards + my position (x+y) + my direction + my health + extra lives + 6 checkpoint positions (filled with -1s if less are placed)
//              + 4 tile entities/tile so 4 * 12 * 12 * (orientation ordinal + typeid + position) + 4 other robot positions + directions (filled with -1s if less are playing)

use crate::datatypes::Position;

//INPUT NODES: handcards as bits + played cards as bits + (my last position erstmal ohne) + my direction as onehot + my health + extra lives (ersma ohne) + next checkpoint position relativ to robot
// + 4 tile entities/tile on a 3 tile radius view point around the robot so 4 * 7 * 7 * (orientation ordinal as onehot + typeid as onehot )
pub const INPUT_NODES: usize = 7*9 + 7*5 + 4 + 1 + 1 * 2 + 4 * 7 * 7 * (4 + 34); //9 + 5 + 2 + 1 + 1 + 1 + 6 * 2 + 4 * 12 * 12 * 4 + 4 * 3;

//Amount of hand cards
pub const OUTPUT_NODES: usize = 9;
pub const HIDDEN_LAYERS: usize = 10;
pub const MUTATION_RATE: f32 = 0.005; //0.1%
pub const GENERATIONS: usize = 10000;
pub const POPULATION_SIZE: i32 = 10;
pub const CHECKPOINTS: &[&[Position]] = &[
    &[Position { x: 7, y: 4 }, Position { x: 7, y: 10 }],
    &[Position { x: 7, y: 10 }, Position { x: 2, y: 10 }],
    &[Position { x: 7, y: 4 }, Position { x: 8, y: 8 }],
    &[Position { x: 1, y: 0 }, Position { x: 3, y: 4 }],
    &[Position { x: 0, y: 10 }, Position { x: 1, y: 0 }],
    &[Position { x: 3, y: 10 }, Position { x: 2, y: 8 }],
    &[Position { x: 3, y: 4 }, Position { x: 1, y: 2 }],
    &[Position { x: 11, y: 4 }, Position { x: 7, y: 4 }],
    &[Position { x: 3, y: 7 }, Position { x: 7, y: 7 }],
    &[Position { x: 7, y: 3 }, Position { x: 3, y: 2 }],
    &[Position { x: 3, y: 2 }, Position { x: 7, y: 3 }],
    // &[Position { x: 7, y: 10 }, Position { x: 2, y: 10 }, Position {x: 1, y: 7}, Position {x: 1, y: 2}, Position {x: 3, y: 2}],
    &[Position { x: 7, y: 4 }, Position { x: 2, y: 8 }],
    // &[Position { x: 1, y: 0 }, Position { x: 7, y: 8 }, Position { x: 1, y: 7 }, Position { x: 7, y: 9 }, Position { x: 10, y: 8 }],
];
pub const ROUND_THRESHOLD: usize = 30;
pub const HIDDEN_NODES: usize = INPUT_NODES / 2;
//x% of neurons will be crossed over on offspring production
pub const PERCENTAGE_RAND_NEURONS_CROSSOVER: f32 = 0.1;
pub const SAVE_GEN_INTERVAL: usize = 20;
