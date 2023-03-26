#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    Start,
    HandOutCards,
    ExecuteCard(usize),
    FactoryState(usize, FactoryState),
    RoundEnd,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum FactoryState {
    ExpressBelt,
    StandartBelt,
    Shover,
    SpinField,
    Press,
    Laser,
}
