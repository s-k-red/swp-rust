use std::collections::HashMap;

use crate::{game_states::gamestates::GameState, components::components::GameStore};

struct Automaton {
    transitions: HashMap<GameState, GameState>,
}

trait StateAction {
    fn on_entry(&self, game_store: &mut GameStore);
}

impl StateAction for GameState {
    fn on_entry(&self, game_store: &mut GameStore) {
        match &self {
            GameState::Start => return,
            GameState::HandOutCards => todo!(),
            GameState::ExecuteCard(card_number) => todo!(),
            GameState::FactoryState(card_number, factory_state) => todo!(),
            GameState::RoundEnd => todo!(),
        }
    }
}