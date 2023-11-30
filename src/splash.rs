use crate::assets::SplashAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashWhileLoadingAssets), setup);
        app.add_systems(OnExit(GameState::SplashWhileLoadingAssets), cleanup);
    }
}

#[derive(Component, Debug)]
pub struct Splash;

fn setup(mut commands: Commands, splash_assets: Res<SplashAssets>) {
    info!("splash setup");

    let texture = splash_assets.loading_splash.clone();

    commands.spawn((
        Name::new("Splash"),
        Splash,
        SpriteBundle {
            texture,
            ..default()
        },
    ));
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Splash>>) {
    info!("splash cleanup");

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
