use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum GameState {
    Start,
    HandOutCards,
    ExecuteCard(usize),
    FactoryState(usize, FactoryState),
    RoundEnd,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum FactoryState {
    ExpressBelt,
    StandartBelt,
    Shover,
    SpinField,
    Press,
    Laser,
}
