mod assets;
mod console;
mod fullscreen;
mod game;
mod input;
mod menus;
mod splash;
mod style;

use bevy::asset::AssetMetaCheck;
use bevy::input::common_conditions::input_toggle_active;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};
use console::ConsoleAction;
use input::GameAction;
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

    // let default_filter = "info,wgpu=error,naga=warn,winit=info,gilrs=info".to_string();
    // let filter_layer = EnvFilter::try_from_default_env()
    //     .or_else(|_| EnvFilter::try_new(&default_filter))
    //     .unwrap();
    //
    // tracing_subscriber::registry()
    //     .with(filter_layer)
    //     .with(
    //         tracing_subscriber::fmt::Layer::new()
    //             .with_ansi(true)
    //             .without_time(), // with_time (or default) won't run because time is not implemented for wasm apparently.
    //     )
    //     .with(console_plugin.clone())
    //     .init();

    let mut app = App::new();

    app.add_state::<GameState>();

    // https://github.com/bevyengine/bevy/issues/10157#issuecomment-1802920833
    app.insert_resource(AssetMetaCheck::Never);

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "{{ project-name }}".to_string(),
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }),
        // Disabling LogPlugin makes wasm logging not work.
        // .build()
        // .disable::<LogPlugin>(),
        WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::P)),
        console_plugin,
        InputManagerPlugin::<GameAction>::default(),
        AudioPlugin,
    ));

    // Internal Plugins
    app.add_plugins((
        assets::AssetsPlugin,
        fullscreen::FullscreenPlugin,
        console::ConsoleHandlerPlugin,
        splash::SplashPlugin,
        menus::MenusPlugin,
        game::GamePlugin,
    ));

    app.add_systems(Startup, (setup_2d_camera, debug_stuff));

    app.run();
}

fn setup_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    info!("setup 2d camera");
}

fn debug_stuff(windows: Query<&Window>) {
    for window in &windows {
        info!("window: {:#?}", window);
    }
}
