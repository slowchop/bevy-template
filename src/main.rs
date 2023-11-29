mod console;
mod game;
mod input;
mod ui;

use crate::game::GamePlugin;
use bevy::prelude::*;
use console::ConsoleAction;
use input::KeyAction;
use leafwing_input_manager::prelude::{Actionlike, InputManagerPlugin};
use slowchop_console::ConsolePlugin;
use strum::EnumString;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, EnumString)]
pub enum GameState {
    #[default]
    LoadingAssets,
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

fn main() {
    let console_plugin = ConsolePlugin::<ConsoleAction>::default();

    let default_filter = "trace,wgpu=error,naga=warn,bevy=info,winit=info,gilrs=info".to_string();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&default_filter))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(tracing_subscriber::fmt::Layer::new().with_ansi(true))
        .with(console_plugin.clone())
        .init();

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    position: WindowPosition::Centered(MonitorSelection::Current),
                    title: "{{ project-name }}".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            console_plugin,
            InputManagerPlugin::<KeyAction>::default(),
            // Internal Plugins
            GamePlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (console::handle_console_actions))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
