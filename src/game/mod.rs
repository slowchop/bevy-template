mod ui;

use crate::assets::GameAssets;
use crate::game::ui::GameUiPlugin;
use crate::input::GameAction;
use crate::GameState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::{ActionState, InputMap};
use leafwing_input_manager::InputManagerBundle;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameUiPlugin);

        app.add_systems(OnEnter(GameState::Playing), setup);
        app.add_systems(Update, (movement).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    info!("game setup");

    let texture = game_assets.player.clone();
    let input_map = GameAction::input_map();
    commands.spawn((
        Name::new("Player"),
        Player,
        InputManagerBundle {
            input_map,
            ..default()
        },
        SpriteBundle {
            texture,
            ..default()
        },
    ));
}

fn movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ActionState<GameAction>), With<Player>>,
) {
    let (mut transform, action_state) = query.single_mut();

    let left = action_state.pressed(GameAction::Left);
    let right = action_state.pressed(GameAction::Right);
    let up = action_state.pressed(GameAction::Up);
    let down = action_state.pressed(GameAction::Down);

    let mut velocity = Vec3::ZERO;
    if left {
        velocity.x -= 1.0;
    }
    if right {
        velocity.x += 1.0;
    }
    if up {
        velocity.y += 1.0;
    }
    if down {
        velocity.y -= 1.0;
    }
    let velocity = velocity.normalize_or_zero() * 500.0 * time.delta_seconds();

    transform.translation += velocity;
}
