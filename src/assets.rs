use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct SplashAssets {
    #[asset(path = "loading-splash.png")]
    pub loading_splash: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct MenuAssets {
    #[asset(key = "menu-background")]
    pub menu_background: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(key = "player")]
    player: Handle<Image>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        // Assets only for the splash screen
        app.add_loading_state(
            LoadingState::new(GameState::LoadingSplashAssets)
                .continue_to_state(GameState::SplashWhileLoadingAssets),
        );
        app.add_collection_to_loading_state::<_, SplashAssets>(GameState::LoadingSplashAssets);

        // The rest of the assets.
        // TODO: Block on menu loading here, and trigger a background load of the game assets.
        app.add_loading_state(
            LoadingState::new(GameState::SplashWhileLoadingAssets)
                .continue_to_state(GameState::Menus),
        );
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
