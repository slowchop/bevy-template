use bevy::prelude::*;

#[derive(Action, Clone, Debug)]
enum ConsoleAction {
    Quit,
}

#[derive(ActionLike, Clone, Debug)]
enum KeyAction {
    Up,
    Down,
    Left,
    Right,

    Run,
    Jump,
    Action1,

    Escape,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    position: WindowPosition::Centered(MonitorSelection::Current),
                    title: "{{project-name}}".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            ConsolePlugin::<ConsoleAction>::default(),
            InputManagerPlugin::<KeyAction>::default(),
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (handle_console_actions))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn handle_console_actions(mut console_actions: EventReader<ConsoleAction>) {
    for action in console_actions.iter() {
        match action {
            ConsoleAction::Quit => std::process::exit(0),
        }
    }
}
