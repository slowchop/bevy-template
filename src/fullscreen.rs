use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

pub struct FullscreenPlugin;

impl Plugin for FullscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeFullscreen>();
        app.add_systems(Update, change);
    }
}

#[derive(Event)]
pub enum ChangeFullscreen {
    Toggle,
    Windowed,
    BorderlessFullscreen,
}

fn change(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut events: EventReader<ChangeFullscreen>,
) {
    let mut window = window.single_mut();

    for mode in events.read() {
        match mode {
            ChangeFullscreen::Toggle => match window.mode {
                WindowMode::Windowed => {
                    window.mode = WindowMode::BorderlessFullscreen;
                }
                WindowMode::BorderlessFullscreen => {
                    window.mode = WindowMode::Windowed;
                }
                WindowMode::Fullscreen => {
                    window.mode = WindowMode::Windowed;
                }
                WindowMode::SizedFullscreen => {
                    window.mode = WindowMode::Windowed;
                }
            },
            ChangeFullscreen::Windowed => {
                window.mode = WindowMode::Windowed;
            }
            ChangeFullscreen::BorderlessFullscreen => {
                window.mode = WindowMode::BorderlessFullscreen;
            }
        }
    }
}
