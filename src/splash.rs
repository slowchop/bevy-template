use crate::assets::SplashAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashWhileLoadingAssets), setup);
    }
}

fn setup(mut commands: Commands, splash_assets: Res<SplashAssets>) {
    info!("splash setup");

    commands.spawn(SpriteBundle {
        texture: splash_assets.loading_splash.clone(),
        ..default()
    });
}
