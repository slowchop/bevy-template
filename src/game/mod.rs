mod ui;

use crate::assets::GameAssets;
use crate::game::ui::GameUiPlugin;
use crate::input::GameAction;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::AudioEmitter;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioTween};
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

#[derive(Component, Deref, DerefMut)]
struct AudioInstanceHolder(Handle<AudioInstance>);

fn setup(mut commands: Commands, game_assets: Res<GameAssets>, audio: Res<Audio>) {
    info!("game setup");

    let texture = game_assets.player.clone();
    let input_map = GameAction::input_map();
    let walk_sound = audio.play(game_assets.walk_sound.clone()).looped().handle();

    commands.spawn((
        Name::new("Player"),
        Player,
        InputManagerBundle {
            input_map,
            ..default()
        },
        SpriteBundle {
            texture,
            transform: Transform::from_scale(Vec3::new(0.2, 0.2, 1.0)),
            ..default()
        },
        AudioEmitter {
            instances: vec![walk_sound.clone()],
        },
        AudioInstanceHolder(walk_sound),
    ));
}

fn movement(
    time: Res<Time>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut query: Query<
        (
            &mut Transform,
            &mut AudioInstanceHolder,
            &ActionState<GameAction>,
        ),
        With<Player>,
    >,
) {
    let (mut transform, mut audio_instance, action_state) = query.single_mut();

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

    if let Some(instance) = audio_instances.get_mut(&**audio_instance) {
        if velocity != Vec3::ZERO {
            instance.resume(AudioTween::default());
        } else {
            instance.pause(AudioTween::default());
        }
    }

    transform.translation += velocity;
}
