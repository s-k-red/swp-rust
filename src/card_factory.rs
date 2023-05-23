use crate::{components::Card, commands::RobotCommand, datatypes::Direction};

pub fn create_card_deck() -> Vec<Card> {
    let mut cards = Vec::new();

    //TURN LEFT
    for i in (70..411).step_by(20) {
        cards.push(Card { id: i, is_movement: false, 
            commands: vec![RobotCommand::Absolute(crate::commands::RobotAction::InPlace(crate::commands::RobotActionInPlace::Turn(Direction{ordinal: 3})))] });        
    }

    //TURN RIGHT
    for i in (80..421).step_by(20) {
        cards.push(Card { id: i, is_movement: false, 
            commands: vec![RobotCommand::Absolute(crate::commands::RobotAction::InPlace(crate::commands::RobotActionInPlace::Turn(Direction{ordinal: 1})))] });        
    }

    //U TURN
    for i in (10..61).step_by(10) {
        cards.push(Card { id: i, is_movement: false, 
            commands: vec![RobotCommand::Absolute(crate::commands::RobotAction::InPlace(crate::commands::RobotActionInPlace::Turn(Direction{ordinal: 1}))),
            RobotCommand::Absolute(crate::commands::RobotAction::InPlace(crate::commands::RobotActionInPlace::Turn(Direction{ordinal: 1})))] });        
    }

    //BACK UP
    for i in (430..481).step_by(10) {
        cards.push(Card { id: i, is_movement: true, 
            commands: vec![RobotCommand::DeclareRelativeMove(Direction { ordinal: 2 })] });        
    }

    //MOVE 1
    for i in (490..661).step_by(10) {
        cards.push(Card { id: i, is_movement: true, 
            commands: vec![RobotCommand::DeclareRelativeMove(Direction { ordinal: 0 })] });        
    }

    //MOVE 2
    for i in (670..781).step_by(10) {
        cards.push(Card { id: i, is_movement: true, 
            commands: vec![RobotCommand::DeclareRelativeMove(Direction { ordinal: 0 }), RobotCommand::DeclareRelativeMove(Direction { ordinal: 0 })] });        
    }

    //MOVE 3
    for i in (790..841).step_by(10) {
        cards.push(Card { id: i, is_movement: true, 
            commands: vec![RobotCommand::DeclareRelativeMove(Direction { ordinal: 0 }), RobotCommand::DeclareRelativeMove(Direction { ordinal: 0 }), 
            RobotCommand::DeclareRelativeMove(Direction { ordinal: 0 })] });        
    }

    cards
}