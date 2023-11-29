use crate::GameState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
        app.add_systems(Update, (playing).run_if(in_state(GameState::Playing)));
    }
}

fn setup() {
    info!("game setup")
}

fn playing() {
    info!("game playing");
}
