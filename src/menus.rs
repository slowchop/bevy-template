use crate::assets::MenuAssets;
use crate::{style, GameState};
use bevy::prelude::*;
use bevy_ui_dsl::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuState {
    #[default]
    MainMenu,
    Credits,

    #[cfg(not(target_arch = "wasm32"))]
    Quit,
}

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menus), setup);
        app.add_systems(OnExit(GameState::Menus), cleanup);
    }
}

#[derive(Component, Debug)]
pub struct Menus;

fn setup(mut commands: Commands, assets: Res<AssetServer>, menu_assets: Res<MenuAssets>) {
    info!("menus setup");

    commands.spawn((
        Name::new("Background"),
        Menus,
        SpriteBundle {
            texture: menu_assets.menu_background.clone(),
            ..default()
        },
    ));

    root(
        (style::node_root, style::node_menu),
        &assets,
        &mut commands,
        |p| {
            text(
                "{{ project-name }}",
                style::text_bundle,
                style::text_style,
                p,
            );

            menu_button(p, "Play");
            menu_button(p, "Credits");

            #[cfg(not(target_arch = "wasm32"))]
            menu_button(p, "Quit");
        },
    );
}

fn menu_button(p: &mut UiChildBuilder, button_text: &str) {
    node(style::node_menu, p, |p| {
        button(style::button, p, |p| {
            text(
                button_text,
                (style::text_bundle, style::button_text),
                style::text_style,
                p,
            );
        });
    });
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Menus>>) {
    info!("menus cleanup");

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
