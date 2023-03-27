use crate::state::{GameState, StateConfig, StateDisplay};
use bevy::prelude::*;
use std::time::{Duration, Instant};

#[derive(Component, Deref)]
pub struct EndSplashTime(pub Instant);

pub fn enter(
    mut commands: Commands,
    state_config: Res<StateConfig>,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
) {
    let StateDisplay::Splash(splash_state) = state_config.get(&state.0).unwrap() else {
        panic!("{state:?} is not a Splash: {state_config:?}");
    };
    dbg!(splash_state);
    let texture = asset_server.load(&splash_state.asset);

    commands.spawn(SpriteBundle {
        texture,
        ..Default::default()
    });

    commands.spawn(EndSplashTime(
        Instant::now() + Duration::from_millis(splash_state.ms),
    ));
}

pub fn update(
    state_config: Res<StateConfig>,
    mut state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    end_splash_time: Query<&EndSplashTime>,
) {
    let StateDisplay::Splash(splash_state) = state_config.get(&state.0).unwrap() else {
        panic!("{state:?} is not a Splash: {state_config:?}");
    };

    let end_time = end_splash_time.single();
    if end_time.0 > Instant::now() {
        return;
    }

    next_state.set(splash_state.next.clone());
}

pub fn exit(mut commands: Commands, mut end_splash_time: Query<Entity, With<EndSplashTime>>) {
    for entity in end_splash_time.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
