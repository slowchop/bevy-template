use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::{Audio, AudioSource};
use iyes_progress::ProgressPlugin;

#[derive(AssetCollection, Resource)]
pub struct SplashAssets {
    #[asset(path = "loading-splash.png")]
    pub loading_splash: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct MenuAssets {
    #[asset(key = "menu-background")]
    pub menu_background: Handle<Image>,
    #[asset(key = "background-music")]
    pub background_music: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(key = "player")]
    pub player: Handle<Image>,

    #[asset(key = "walk-sound")]
    pub walk_sound: Handle<AudioSource>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: This is a bit crashy. Assets don't seem to be actually loading.
        // app.add_plugins(
        //     ProgressPlugin::new(GameState::LoadingSplashAssets)
        //         .continue_to(GameState::SplashWhileLoadingAssets),
        // );
        // app.add_plugins(
        //     ProgressPlugin::new(GameState::SplashWhileLoadingAssets).continue_to(GameState::Menus),
        // );

        // Assets only for the splash screen
        app.add_loading_state(
            LoadingState::new(GameState::LoadingSplashAssets)
                .continue_to_state(GameState::SplashWhileLoadingAssets),
        );
        app.add_loading_state(LoadingState::new(GameState::LoadingSplashAssets));
        app.add_collection_to_loading_state::<_, SplashAssets>(GameState::LoadingSplashAssets);

        // The rest of the assets.
        app.add_loading_state(
            LoadingState::new(GameState::SplashWhileLoadingAssets)
                .continue_to_state(GameState::Menus),
        );
        app.add_loading_state(LoadingState::new(GameState::SplashWhileLoadingAssets));
        app.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::SplashWhileLoadingAssets,
            "menu.assets.ron",
        );
        app.add_collection_to_loading_state::<_, MenuAssets>(GameState::SplashWhileLoadingAssets);
        app.add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            GameState::SplashWhileLoadingAssets,
            "game.assets.ron",
        );
        app.add_collection_to_loading_state::<_, GameAssets>(GameState::SplashWhileLoadingAssets);
    }
}
