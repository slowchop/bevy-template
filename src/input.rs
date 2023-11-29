use bevy::prelude::Reflect;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum KeyAction {
    Up,
    Down,
    Left,
    Right,

    Run,
    Jump,
    Action1,

    Escape,
}
