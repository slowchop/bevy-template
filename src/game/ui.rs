use crate::menus::{CreditsButton, MainMenu, Menus, PlayButton, QuitButton};
use crate::{style, GameState};
use bevy::prelude::*;
use bevy_ui_dsl::{nodei, rooti, texti};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup);
        app.add_systems(Update, (update_ui).run_if(in_state(GameState::Playing)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("game ui setup");

    rooti(
        (style::node_root, style::node_game_ui),
        &*asset_server,
        &mut commands,
        (Name::new("Game UI Root"), Menus),
        |p| {
            nodei(style::node_game_top_ui, Name::new("Top"), p, |p| {
                texti(
                    "100 / 100",
                    style::text_bundle,
                    style::text_style,
                    Name::new("Health"),
                    p,
                );
            });

            nodei(
                style::node_game_ui_invisible_stretchy_bit,
                Name::new("Invisible"),
                p,
                |_| {},
            );

            nodei(style::node_game_bottom_ui, Name::new("Bottom"), p, |p| {
                texti(
                    "No Weapons LOL GL",
                    style::text_bundle,
                    style::text_style,
                    Name::new("LOL"),
                    p,
                );
            });
        },
    );
}

fn update_ui() {}
