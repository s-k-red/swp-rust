use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameState {
    Start,
    HandOutCards,
    ExecuteCard(usize),
    FactoryState(usize, FactoryState),
    RoundEnd,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactoryState {
    ExpressBelt,
    StandartBelt,
    Shover,
    SpinField,
    Press,
    Laser,
}
