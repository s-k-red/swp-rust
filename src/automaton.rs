use std::vec;

use crate::{
    components::{Board, Card, GameStore, Player, Robot},
    game_states::{FactoryState, GameState},
    resolve_movement::{resolve_card_movement, resolve_factory_movement},
    scheduled_commands::{execute, execute_non_moves, ScheduledActions},
};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub const AUTOMATON_SIZE: usize = 36;
pub const AUTOMATON_STATES: [GameState; AUTOMATON_SIZE] = build_automaton_states();
pub const AUTOMATON: GameAutomaton<AUTOMATON_SIZE> = GameAutomaton::<AUTOMATON_SIZE> {
    state_transitions: AUTOMATON_STATES,
};

const fn build_automaton_states() -> [GameState; AUTOMATON_SIZE] {
    let mut ret = [GameState::Start; AUTOMATON_SIZE];
    let mut i = 0;
    while i < 5 {
        ret[i * 7] = GameState::ExecuteCard(i);
        let mut j = 0;
        while j < 6 {
            ret[i * 7 + j + 1] = GameState::FactoryState(
                i,
                match j {
                    0 => FactoryState::ExpressBelt,
                    1 => FactoryState::StandartBelt,
                    2 => FactoryState::Shover,
                    3 => FactoryState::SpinField,
                    4 => FactoryState::Press,
                    5 => FactoryState::Laser,
                    _ => unreachable!(),
                },
            );
            j += 1;
        }
        i += 1;
    }
    ret[35] = GameState::RoundEnd;
    ret
}

pub struct GameAutomaton<const N: usize> {
    state_transitions: [GameState; N], //without HandOutCards
}
impl Default for GameAutomaton<AUTOMATON_SIZE> {
    fn default() -> Self {
        Self {
            state_transitions: AUTOMATON_STATES,
        }
    }
}
impl<const N: usize> GameAutomaton<N> {
    pub fn hand_out_cards(game_store: &mut GameStore) -> Option<Vec<String>> {
        GameState::HandOutCards.on_entry(
            &mut game_store.robots,
            &game_store.card_deck,
            &game_store.board,
            &mut game_store.players,
            game_store.highest_checkpoint,
        )
    }

    pub fn round_trip(&self, game_store: &mut GameStore) -> Option<Vec<String>> {
        for game_state in &self.state_transitions {
            if let Some(winners) = game_state.on_entry(
                &mut game_store.robots,
                &game_store.card_deck,
                &game_store.board,
                &mut game_store.players,
                game_store.highest_checkpoint,
            ) {
                return Some(winners);
            }
        }
        None
    }
}

trait StateAction {
    fn on_entry(
        &self,
        robots: &mut [Robot],
        card_deck: &[Card],
        board: &Board,
        players: &mut [Player],
        highest_checkpoint: usize,
    ) -> Option<Vec<String>>;
}

impl StateAction for GameState {
    fn on_entry(
        &self,
        robots: &mut [Robot],
        card_deck: &[Card],
        board: &Board,
        players: &mut [Player],
        highest_checkpoint: usize,
    ) -> Option<Vec<String>> {
        match &self {
            GameState::Start => None,
            GameState::HandOutCards => {
                let locked_cards = players
                    .iter()
                    .flat_map(|player| {
                        let locked = (robots
                            .iter()
                            .find(|robot| robot.user_name == player.user_name)
                            .filter(|robot| robot.alive && robot.hp > 0 && robot.hp < 6)?
                            .hp
                            - 1) as usize;
                        Some(&(player.cards_played)[locked..5])
                    })
                    .flatten()
                    .collect::<Vec<&Card>>();
                let mut cards = card_deck
                    .iter()
                    .filter(|card| !locked_cards.contains(card))
                    .cloned()
                    .collect::<Vec<Card>>();

                cards.shuffle(&mut thread_rng());
                for player in players {
                    let robot = robots
                        .iter()
                        .find(|robot| robot.user_name == player.user_name)
                        .expect("player has no robot");
                    if !robot.alive || robot.hp <= 1 {
                        continue;
                    }
                    loop {
                        if cards
                            .iter()
                            .take((robot.hp - 1) as usize)
                            .any(|card| card.is_movement)
                        {
                            break;
                        }
                        cards.shuffle(&mut thread_rng());
                    }
                    player.cards_in_hand = cards.drain(0..(robot.hp as usize)).collect();
                }
                None
            }
            GameState::ExecuteCard(register_number) => {
                let mut cards = players
                    .iter()
                    .filter_map(|player| {
                        player
                            .cards_played
                            .get(*register_number)
                            .map(|c| (player, c))
                    })
                    .collect::<Vec<(&Player, &Card)>>();
                cards.sort_by_key(|(_, card)| card.id);

                for (player, card) in cards {
                    for cmd in &card.commands {
                        robots.sort_unstable_by_key(|robot| (robot.user_name != player.user_name));

                        let (robot, rest) = robots.split_at_mut(1);
                        let sh = robot
                            .into_iter()
                            .map(|robot| {
                                let mut actions = ScheduledActions::new(robot);
                                actions.push_and_convert(cmd.clone());
                                actions
                            })
                            .next()
                            .expect("expected one matching robot");
                        let sh = execute_non_moves(sh);
                        let robot_actions = resolve_card_movement(
                            sh,
                            rest.into_iter().collect::<Vec<&mut Robot>>(),
                            board,
                            self,
                        );
                        for robot_action in robot_actions {
                            execute(robot_action);
                        }
                        if let Some(winners) = calulate_winers(robots, highest_checkpoint) {
                            return Some(winners);
                        }
                    }
                }
                None
            }
            GameState::FactoryState(_, _) => {
                let robot_actions = calculate_actions_from_tile_entities(self, robots, board);
                let robot_moves = robot_actions.into_iter().map(execute_non_moves).collect();
                let robot_actions = resolve_factory_movement(robot_moves, board, self);
                for robot_action in robot_actions {
                    execute(robot_action);
                }
                calulate_winers(robots, highest_checkpoint)
            }
            GameState::RoundEnd => {
                let robot_actions = calculate_actions_from_tile_entities(self, robots, board);
                robot_actions.into_iter().for_each(|action| {
                    execute_non_moves(action);
                });

                let mut occupied = robots
                    .iter()
                    .filter(|robot| robot.alive)
                    .map(|robot| robot.position)
                    .collect::<Vec<_>>();
                for robot in robots
                    .iter_mut()
                    .filter(|robot| !robot.alive && robot.safety_copy_amount > 0)
                    .sorted_by_key(|robot| robot.safety_copy_number)
                {
                    if let Some(winners) = robot.respawn(
                        board,
                        &mut occupied,
                        GameState::RoundEnd,
                        highest_checkpoint,
                    ) {
                        return Some(vec![winners]);
                    }
                }
                None
            }
        }
    }
}

fn calculate_actions_from_tile_entities<'a>(
    game_state: &GameState,
    robots: &'a mut [Robot],
    board: &Board,
) -> Vec<ScheduledActions<'a>> {
    let mut actions = robots
        .iter_mut()
        .map(ScheduledActions::new)
        .collect::<Vec<_>>();

    let indirect_entities = board
        .indirect_tile_eintities
        .get(game_state)
        .cloned()
        .unwrap_or_default();

    for indirect_entity in indirect_entities {
        actions = indirect_entity.convert(board, actions);
    }

    for action in &mut actions {
        if let Some(tile_actions) = board
            .direct_tile_eintities
            .get(game_state)
            .and_then(|all_active| all_active.get(&action.robot.position))
        {
            tile_actions
                .iter()
                .for_each(|tile_action| action.push(tile_action.clone()));
        }
    }

    actions
}

pub fn calulate_winers(robots: &[Robot], highest_checkpoint: usize) -> Option<Vec<String>> {
    let winners = robots
        .iter()
        .filter(|robot| robot.greatest_checkpoint_reached == highest_checkpoint)
        .map(|robot| robot.user_name.clone())
        .collect::<Vec<_>>();
    if !winners.is_empty() {
        Some(winners)
    } else if robots
        .iter()
        .all(|robot| !robot.alive && robot.safety_copy_amount == 0)
    {
        //nobody wins, tmp solution:
        Some(vec![])
    } else {
        None
    }
    /*
    //winning by last one standing:
    else if self.robots.iter().filter(|r|r.alive).count() == 1{
        Some(vec![self.robots.iter().find(|r|r.alive).unwrap().user_name.clone()])
    }
    //winning by other means than reaching the last checkpoint bad if training with only one robot, as it promotes suicide
    else if self.robots.iter().all(|robot| !robot.alive && robot.safety_copy_amount == 0) {
       Some(
           self.robots
               .iter()
               .max_set_by(|robot, other_robot| {
                   robot
                       .greatest_checkpoint_reached
                       .cmp(&other_robot.greatest_checkpoint_reached)
               })
               .iter()
               .map(|robot| robot.user_name.clone())
               .collect::<Vec<_>>(),
       )
    }
     */
}
