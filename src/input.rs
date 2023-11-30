use bevy::prelude::{KeyCode, Reflect};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GameAction {
    Up,
    Down,
    Left,
    Right,

    Run,
    Jump,
    Action1,

    Escape,
}

impl GameAction {
    pub fn input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(KeyCode::W, GameAction::Up);
        input_map.insert(KeyCode::Up, GameAction::Up);
        input_map.insert(KeyCode::S, GameAction::Down);
        input_map.insert(KeyCode::Down, GameAction::Down);
        input_map.insert(KeyCode::A, GameAction::Left);
        input_map.insert(KeyCode::Left, GameAction::Left);
        input_map.insert(KeyCode::D, GameAction::Right);
        input_map.insert(KeyCode::Right, GameAction::Right);

        input_map
    }
}
