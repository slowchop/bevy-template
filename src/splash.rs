use crate::assets::SplashAssets;
use crate::GameState;
use bevy::prelude::*;
use iyes_progress::ProgressCounter;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        let state = GameState::SplashWhileLoadingAssets;

        app.add_systems(OnEnter(state), setup);
        app.add_systems(OnExit(state), cleanup);
        app.add_systems(Update, progress.run_if(in_state(state)));
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

fn progress(progress: Option<Res<ProgressCounter>>) {
    let Some(progress) = progress else { return };

    info!("progress: {:?}", progress.progress());
}
