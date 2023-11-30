use crate::GameState;
use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
        app.add_systems(Update, (update_ui).run_if(in_state(GameState::Playing)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("game ui setup");
}

fn update_ui() {
    info!("game ui update");
}
