use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameState {
    Start,
    HandOutCards,
    ExecuteCard(usize),
    FactoryState(usize, FactoryState),
    RoundEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactoryState {
    ExpressBelt,
    StandartBelt,
    Shover,
    SpinField,
    Press,
    Laser,
}
