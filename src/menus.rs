use crate::assets::MenuAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menus), setup);
        app.add_systems(OnExit(GameState::Menus), cleanup);
    }
}

#[derive(Component, Debug)]
pub struct Menus;

fn setup(mut commands: Commands, menu_assets: Res<MenuAssets>) {
    info!("menus setup");

    commands.spawn((
        Name::new("Background"),
        Menus,
        SpriteBundle {
            texture: menu_assets.menu_background.clone(),
            ..default()
        },
    ));
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Menus>>) {
    info!("menus cleanup");

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
