use crate::assets::MenuAssets;
use crate::{style, GameState};
use bevy::app::AppExit;
use bevy::asset::ErasedAssetLoader;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
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
        app.add_state::<MenuState>();
        app.add_systems(OnEnter(GameState::Menus), (setup, start_background_audio));
        app.add_systems(OnExit(GameState::Menus), cleanup);
        app.add_systems(
            Update,
            main_menu_handler.run_if(in_state(MenuState::MainMenu)),
        );
    }
}

#[derive(Component, Debug)]
pub struct Menus;

#[derive(Component, Debug)]
pub enum MainMenu {
    Play,
    Credits,
    #[cfg(not(target_arch = "wasm32"))]
    Quit,
}

#[derive(Component, Debug)]
pub struct PlayButton;

#[derive(Component, Debug)]
pub struct CreditsButton;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Component, Debug)]
pub struct QuitButton;

// Running this on the load screen really goes chunky. Run it after the audio has loaded.
pub fn start_background_audio(audio: Res<Audio>, menu_assets: Res<MenuAssets>) {
    audio.play(menu_assets.background_music.clone());
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, menu_assets: Res<MenuAssets>) {
    info!("menus setup");

    commands.spawn((
        Name::new("Background"),
        Menus,
        SpriteBundle {
            texture: menu_assets.menu_background.clone(),
            ..default()
        },
    ));

    rooti(
        (style::node_root, style::node_menu),
        &*asset_server,
        &mut commands,
        (Name::new("Menu Root"), Menus),
        |p| {
            texti(
                "{{ project-name }}",
                style::text_bundle,
                style::text_style,
                Name::new("Title"),
                p,
            );

            menu_button(p, "Play", (PlayButton, MainMenu::Play));
            menu_button(p, "Credits", (CreditsButton, MainMenu::Credits));

            #[cfg(not(target_arch = "wasm32"))]
            menu_button(p, "Quit", (QuitButton, MainMenu::Quit));
        },
    );
}

fn menu_button(p: &mut UiChildBuilder, button_text: &str, extras: impl Bundle) {
    nodei(
        style::node_menu,
        Name::new(button_text.to_string()),
        p,
        |p| {
            buttoni(style::button, extras, p, |p| {
                text(
                    button_text,
                    (style::text_bundle, style::button_text),
                    style::text_style,
                    p,
                );
            });
        },
    );
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Menus>>) {
    info!("menus cleanup");

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn main_menu_handler(
    interactions: Query<(&MainMenu, &Interaction), Changed<Interaction>>,
    mut state: ResMut<NextState<GameState>>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    for (menu, interaction) in &interactions {
        match (menu, interaction) {
            (MainMenu::Play, Interaction::Pressed) => {
                info!("Play clicked");
                state.set(GameState::Playing);
            }
            (MainMenu::Credits, Interaction::Pressed) => {
                info!("Credits clicked");
            }
            #[cfg(not(target_arch = "wasm32"))]
            (MainMenu::Quit, Interaction::Pressed) => {
                info!("Quit clicked");
                app_exit_writer.send(AppExit);
            }
            (_, Interaction::Hovered) => {}
            (_, Interaction::None) => {}
        }
    }
}
