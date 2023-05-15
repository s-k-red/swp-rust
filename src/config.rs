//INPUT NODES: Amount of cards + played cards + my position (x+y) + my direction + my health + extra lives + 6 checkpoint positions (filled with -1s if less are placed)
//              + 4 tile entities/tile so 4 * 12 * 12 * (orientation ordinal + typeid + position) + 4 other robot positions + directions (filled with -1s if less are playing)


//INPUT NODES: Amount of cards + played cards + my position (x+y) + my last position + my direction + my health + extra lives + next checkpoint position (filled with -1s if less are placed)
//              + 4 tile entities/tile so 4 * 12 * 12 * (orientation ordinal + typeid ) + 4 other robot positions + directions (filled with -1s if less are playing)
pub const INPUT_NODES: usize = 9 + 5 + 2 + 2 + 1 + 1 + 1 + 1 * 2 + 4 * 12 * 12 * 2 + 4 * 3;//9 + 5 + 2 + 1 + 1 + 1 + 6 * 2 + 4 * 12 * 12 * 4 + 4 * 3;

//Amount of hand cards
pub const OUTPUT_NODES: usize = 9;
pub const HIDDEN_LAYERS: usize = 1;
pub const MUTATION_RATE: f64 = 0.1; //10%
pub const GENERATIONS: usize = 30;
pub const PUPULATION_SIZE: i32 = 30;