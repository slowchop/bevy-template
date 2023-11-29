mod assets;
mod console;
mod game;
mod input;
mod splash;
mod ui;

use bevy::input::common_conditions::input_toggle_active;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
    /// Load the splash screen first.
    #[default]
    LoadingSplashAssets,

    /// Load the rest of the assets.
    SplashWhileLoadingAssets,

    //
    Menus,
    Playing,
    Paused,
    GameOver,
}

fn main() {
    let console_plugin = ConsolePlugin::<ConsoleAction>::default();

    let default_filter =
        "debug,bevy_app=info,bevy_ecs=info,wgpu=error,naga=warn,winit=info,gilrs=info".to_string();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&default_filter))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(tracing_subscriber::fmt::Layer::new().with_ansi(true))
        .with(console_plugin.clone())
        .init();

    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    position: WindowPosition::Centered(MonitorSelection::Current),
                    title: "{{ project-name }}".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .build()
            .disable::<LogPlugin>(),
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F1)),
        console_plugin,
        InputManagerPlugin::<KeyAction>::default(),
    ));

    // Internal Plugins
    app.add_plugins((
        assets::AssetsPlugin,
        console::ConsoleHandlerPlugin,
        splash::SplashPlugin,
        game::GamePlugin,
    ));

    app.add_state::<GameState>();
    app.add_systems(Startup, setup_2d_camera);

    app.add_systems(Update, debug);

    app.run();
}

fn setup_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn debug(state: Res<State<GameState>>) {
    info!("Current state: {:?}", state);
}
