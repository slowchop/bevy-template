use crate::fullscreen::ChangeFullscreen;
use crate::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;
use slowchop_console::Actions;
use std::str::FromStr;

pub struct ConsoleHandlerPlugin;

impl Plugin for ConsoleHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_console_actions);
    }
}

#[derive(Actions, Event, Clone, Debug)]
pub enum ConsoleAction {
    Quit,

    // TODO: Similar to below, implement enums.
    SetState(String),

    // TODO: implement enums in slowchop_console so we can do this:
    // Fullscreen(FullscreenMode), i.e. fullscreen toggle
    FullscreenToggle,
    Fullscreen,
    Windowed,
}

fn handle_console_actions(
    mut console_actions: EventReader<ConsoleAction>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_writer: EventWriter<AppExit>,
    mut fullscreen_writer: EventWriter<ChangeFullscreen>,
) {
    for action in console_actions.read() {
        match action {
            ConsoleAction::Quit => {
                app_exit_writer.send(AppExit);
            }
            ConsoleAction::SetState(new_state) => match GameState::from_str(new_state) {
                Ok(state) => {
                    info!("State change: {:?}", state);
                    next_state.set(state);
                }
                Err(e) => {
                    error!("invalid state: {:?}", e);
                }
            },
            ConsoleAction::FullscreenToggle => {
                fullscreen_writer.send(ChangeFullscreen::Toggle);
            }
            ConsoleAction::Fullscreen => {
                fullscreen_writer.send(ChangeFullscreen::BorderlessFullscreen);
            }
            ConsoleAction::Windowed => {
                fullscreen_writer.send(ChangeFullscreen::Windowed);
            }
        }
    }
}
