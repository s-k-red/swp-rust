#[cfg(test)]
use crate::{
    components::{Board, Robot, Wall},
    datatypes::{Direction, Position},
    resolve_movement::{resolve_card_movement, resolve_factory_movement},
    scheduled_commands::{execute, ScheduledMove},
};
#[cfg(test)]
use std::collections::{HashMap, HashSet};
#[cfg(test)]
fn setup(dim: i32, walls: HashSet<Wall>) -> Board {
    let mut inbounds = HashSet::new();
    for x in 0..dim {
        for y in 0..dim {
            inbounds.insert(Position { x, y });
        }
    }
    Board {
        direct_tile_eintities: HashMap::new(),
        indirect_tile_eintities: HashMap::new(),
        on_entry_tile_eintities: HashMap::new(),
        walls,
        pos_inbounds: inbounds,
    }
}

#[test]
fn card_move_foreward() {
    let board = setup(2, HashSet::new());
    let mut robot = Robot::new("test".into(), Position { x: 0, y: 0 });
    let shed = ScheduledMove {
        robot: &mut robot,
        mov: Some(Direction::new(0)),
    };

    let ret = resolve_card_movement(
        shed,
        vec![],
        &board,
        &crate::game_states::GameState::ExecuteCard(0),
    );
    for action in ret {
        execute(action);
    }
    assert_eq!(robot.position, Position { x: 0, y: 1 })
}

#[test]
fn factory_move_foreward() {
    let board = setup(2, HashSet::new());
    let mut robot = Robot::new("test".into(), Position { x: 0, y: 0 });
    let shed = ScheduledMove {
        robot: &mut robot,
        mov: Some(Direction::new(0)),
    };

    let ret = resolve_factory_movement(
        vec![shed],
        &board,
        &crate::game_states::GameState::FactoryState(
            0,
            crate::game_states::FactoryState::ExpressBelt,
        ),
    );
    for action in ret {
        execute(action);
    }
    assert_eq!(robot.position, Position { x: 0, y: 1 })
}

#[test]
fn test() {
    let mut wall = HashSet::new();
    wall.insert(Wall(Position { x: 0, y: 0 }, Position { x: 0, y: 1 }));
    let board = setup(2, wall);
    let mut robot = Robot::new("test".into(), Position { x: 0, y: 0 });
    let shed = ScheduledMove {
        robot: &mut robot,
        mov: Some(Direction::new(0)),
    };

    let ret = resolve_factory_movement(
        vec![shed],
        &board,
        &crate::game_states::GameState::FactoryState(
            0,
            crate::game_states::FactoryState::ExpressBelt,
        ),
    );
    for action in ret {
        execute(action);
    }
    assert_eq!(robot.position, Position { x: 0, y: 0 })
}

#[test]
fn shove() {
    let board = setup(3, HashSet::new());
    let mut robot = Robot::new("test".into(), Position { x: 0, y: 0 });
    let mut robot1 = Robot::new("test1".into(), Position { x: 0, y: 1 });
    let shed = ScheduledMove {
        robot: &mut robot,
        mov: Some(Direction::new(0)),
    };

    let ret = resolve_card_movement(
        shed,
        vec![&mut robot1],
        &board,
        &crate::game_states::GameState::ExecuteCard(0),
    );
    for action in ret {
        execute(action);
    }
    assert_eq!(robot.position, Position { x: 0, y: 1 });
    assert_eq!(robot1.position, Position { x: 0, y: 2 });
}

#[test]
fn collide() {
    let board = setup(3, HashSet::new());
    let mut robot = Robot::new("test".into(), Position { x: 0, y: 0 });
    let mut robot1 = Robot::new("test1".into(), Position { x: 0, y: 1 });
    let shed = ScheduledMove {
        robot: &mut robot,
        mov: Some(Direction::new(0)),
    };
    let shed1 = ScheduledMove {
        robot: &mut robot1,
        mov: None,
    };

    let ret = resolve_factory_movement(
        vec![shed, shed1],
        &board,
        &crate::game_states::GameState::ExecuteCard(0),
    );
    for action in ret {
        execute(action);
    }
    assert_eq!(robot.position, Position { x: 0, y: 0 });
    assert_eq!(robot1.position, Position { x: 0, y: 1 });
}

#[test]
fn kill() {
    let board = setup(1, HashSet::new());
    let mut robot = Robot::new("test".into(), Position { x: 0, y: 0 });
    let shed = ScheduledMove {
        robot: &mut robot,
        mov: Some(Direction::new(0)),
    };

    let ret = resolve_factory_movement(
        vec![shed],
        &board,
        &crate::game_states::GameState::FactoryState(
            0,
            crate::game_states::FactoryState::ExpressBelt,
        ),
    );
    for action in ret {
        execute(action);
    }
    assert!(!robot.alive)
}
#[test]
fn kill1() {
    let board = setup(1, HashSet::new());
    let mut robot = Robot::new("test".into(), Position { x: 0, y: 0 });
    let shed = ScheduledMove {
        robot: &mut robot,
        mov: Some(Direction::new(0)),
    };

    let ret = resolve_card_movement(
        shed,
        vec![],
        &board,
        &crate::game_states::GameState::FactoryState(
            0,
            crate::game_states::FactoryState::ExpressBelt,
        ),
    );
    for action in ret {
        execute(action);
    }
    assert!(!robot.alive)
}
#[test]
fn game_end() {
    let mut board = setup(2, HashSet::new());
    board.add_checkpoints(vec![Position { x: 0, y: 0 }, Position { x: 0, y: 1 }]);
    let mut robot = Robot::new("test".into(), Position { x: 0, y: 0 });
    let shed = ScheduledMove {
        robot: &mut robot,
        mov: Some(Direction::new(0)),
    };

    let ret = resolve_card_movement(
        shed,
        vec![],
        &board,
        &crate::game_states::GameState::FactoryState(
            0,
            crate::game_states::FactoryState::ExpressBelt,
        ),
    );
    for action in ret {
        execute(action);
    }
    dbg!(robot);
}
