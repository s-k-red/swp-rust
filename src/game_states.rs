#![allow(dead_code)]
#![allow(unused_variables)]

pub(crate) mod gamestates {

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum GameState {
        Start,
        HandOutCards,
        ExecuteCard(u32),
        FactoryState(u32, FactoryState),
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
}
