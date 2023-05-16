use crate::{
    commands::TileEntity,
    components::{Board, Card, GameStore, GameStoreBuilder, Player, Robot},
    datatypes::Position,
};

pub fn convert(
    board: Vec<TileEntity>,
    names: Vec<String>,
    card_deck: Vec<Card>,
    starting_pos: Position,
) -> GameStore {
    let res = GameStoreBuilder::default()
        .players(names.clone().into_iter().map(Player::new).collect())
        .robots(
            names
                .into_iter()
                .map(|user_name| Robot::new(user_name, starting_pos))
                .collect(),
        )
        .card_deck(card_deck)
        .board(Board::new(board))
        .highest_checkpoint(3) //TODO
        .build();
    match res {
        Ok(game_store) => game_store,
        Err(err) => panic!("{}", err),
    }
}
