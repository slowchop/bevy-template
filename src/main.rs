use crate::settings::GameSettings;
use crate::state::GameState;
use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::{WindowLevel, WindowMode, WindowResolution};
use bevy::DefaultPlugins;
use bevy_egui::egui::util::undoer::Settings;
use bevy_egui::EguiPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;

pub mod controller;
pub mod input;
pub mod settings;
pub mod setup;
pub mod state;
pub mod ui;

fn main() {
    let mut app = App::new();

    let game_settings = GameSettings {
        window_mode: WindowMode::BorderlessFullscreen,
        resolution: (1920f32, 1080f32),
        show_debug_ui: true,
        show_stats: true,
    };

    // Bevy plugins
    app.add_plugins(bevy_plugins(&game_settings));

    // Third party plugins
    app.add_plugin(EguiPlugin);
    app.add_plugin(DebugLinesPlugin::default());

    // Game resources
    app.insert_resource(game_settings);
    app.add_state::<GameState>();

    // Game startup
    app.add_startup_system(setup::setup);
    // TODO: app.add_system(setup::enumerate_resolutions);

    app.add_startup_system(ui::stats::setup_stats);

    // Fixed frame rate systems
    app.add_systems(().in_schedule(CoreSchedule::FixedUpdate));

    // Video frame rate systems
    app.add_systems((ui::stats::update_stats,));

    app.run();
}

fn bevy_plugins(settings: &GameSettings) -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::Centered(MonitorSelection::Current),
                resolution: WindowResolution::new(settings.resolution.0, settings.resolution.1),
                mode: settings.window_mode,
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
}
