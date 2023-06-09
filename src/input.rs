use crate::state;
use crate::state::MenuState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Resource, Deref, DerefMut)]
pub struct KeyboardInputMap(HashMap<KeyCode, Action>);

#[derive(Resource, Deref, DerefMut)]
pub struct MouseButtonInputMap(HashMap<MouseButton, Action>);

#[derive(Resource, Deref, DerefMut)]
pub struct InputStates(HashMap<Action, InputState>);

#[derive(Default, Copy, Clone)]
pub struct InputState {
    pub just_pressed: bool,
    pub just_released: bool,
    pub is_pressed: bool,
}

#[derive(Eq, PartialEq)]
pub enum EventState {
    Pressed,
    Released,
}

pub struct ActionEvent {
    pub action: Action,
    pub state: EventState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Jump,
    PrimaryAction,
    SecondaryAction,
    Use,
    Special,
    Confirm,
    Escape,
}

pub fn setup(mut commands: Commands) {
    let mut keyboard_input_map = HashMap::new();
    let mut mouse_button_input_map = HashMap::new();

    keyboard_input_map.insert(KeyCode::Up, Action::Up);
    keyboard_input_map.insert(KeyCode::Down, Action::Down);
    keyboard_input_map.insert(KeyCode::Left, Action::Left);
    keyboard_input_map.insert(KeyCode::Right, Action::Right);

    keyboard_input_map.insert(KeyCode::W, Action::Up);
    keyboard_input_map.insert(KeyCode::S, Action::Down);
    keyboard_input_map.insert(KeyCode::A, Action::Left);
    keyboard_input_map.insert(KeyCode::D, Action::Right);

    keyboard_input_map.insert(KeyCode::Return, Action::Confirm);
    keyboard_input_map.insert(KeyCode::Space, Action::Jump);
    keyboard_input_map.insert(KeyCode::E, Action::Use);
    keyboard_input_map.insert(KeyCode::Q, Action::Special);

    mouse_button_input_map.insert(MouseButton::Left, Action::PrimaryAction);
    mouse_button_input_map.insert(MouseButton::Right, Action::SecondaryAction);

    commands.insert_resource(KeyboardInputMap(keyboard_input_map));
    commands.insert_resource(MouseButtonInputMap(mouse_button_input_map));

    commands.insert_resource(InputStates(HashMap::new()));

    // Add Event type to world
    commands.insert_resource(Events::<ActionEvent>::default());
}

pub fn process_keyboard_input(
    mut input_states: ResMut<InputStates>,
    keyboard_input_map: Res<KeyboardInputMap>,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut input_action_writer: EventWriter<ActionEvent>,
) {
    // Set all "just_pressed" and "just_released" to false.
    for state in input_states.values_mut() {
        state.just_pressed = false;
        state.just_released = false;
    }

    for event in keyboard_events.iter() {
        let Some(key_code) = event.key_code else {
            continue;
        };
        let Some(action) = keyboard_input_map.get(&key_code) else {
            continue;
        };
        let (input_state, event_state) = match event.state {
            ButtonState::Pressed => (
                InputState {
                    is_pressed: true,
                    just_pressed: true,
                    just_released: false,
                },
                EventState::Pressed,
            ),
            ButtonState::Released => (
                InputState {
                    is_pressed: false,
                    just_pressed: false,
                    just_released: true,
                },
                EventState::Released,
            ),
        };

        input_states.insert(*action, input_state);

        input_action_writer.send(ActionEvent {
            action: *action,
            state: event_state,
        });

        info!("Keyboard event: {:?}", event);
    }
}

pub fn process_mouse_input(
    mut input_states: ResMut<InputStates>,
    mouse_button_input_map: Res<MouseButtonInputMap>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut input_action_writer: EventWriter<ActionEvent>,
) {
    // Set all "just_pressed" and "just_released" to false.
    for state in input_states.values_mut() {
        state.just_pressed = false;
        state.just_released = false;
    }

    for event in mouse_button_events.iter() {
        let Some(action) = mouse_button_input_map.get(&event.button) else {
            continue;
        };
        let (input_state, event_state) = match event.state {
            ButtonState::Pressed => (
                InputState {
                    is_pressed: true,
                    just_pressed: true,
                    just_released: false,
                },
                EventState::Pressed,
            ),
            ButtonState::Released => (
                InputState {
                    is_pressed: false,
                    just_pressed: false,
                    just_released: true,
                },
                EventState::Released,
            ),
        };

        input_states.insert(*action, input_state);

        input_action_writer.send(ActionEvent {
            action: *action,
            state: event_state,
        });

        info!("Mouse event: {:?}", event);
    }
}
