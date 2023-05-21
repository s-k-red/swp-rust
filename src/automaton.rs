use std::vec;

use crate::{
    components::{Board, Card, GameStore, Player, Robot},
    game_states::GameState,
    resolve_movement::{resolve_card_movement, resolve_factory_movement},
    scheduled_commands::{execute, execute_non_moves, ScheduledActions},
};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct GameAutomaton {
    state_transitions: Vec<GameState>, //without HandOutCards
}

impl GameAutomaton {
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
                        Some(&(player.cards_played)[locked..4])
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
                    if !robot.alive || robot.hp == 0 {
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
                    player.cards_in_hand = cards.drain(0..(robot.hp - 1) as usize).collect();
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
                        let robot_actions = vec![robots
                            .iter_mut()
                            .find(|robot| robot.user_name == player.user_name)
                            .map(|robot| {
                                if robot.user_name == player.user_name {
                                    let mut actions = ScheduledActions::new(robot);
                                    actions.push_and_convert(cmd.clone());
                                }
                                ScheduledActions::new(robot)
                            })
                            .unwrap()];
                        let robot_moves =
                            robot_actions.into_iter().map(execute_non_moves).collect();
                        let robot_actions = resolve_card_movement(robot_moves, board, self);
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
