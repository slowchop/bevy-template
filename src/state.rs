use bevy::prelude::*;

/// This has to match the assets/state.yaml file.
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Settings,
    Credits,
    Game,
}

pub struct StateConfig(Vec<StateItem>);

pub struct StateItem {
    state: GameState,
    display: StateDisplay,
}

pub enum StateDisplay {
    Splash(SplashState),
    Menu(MenuState),
}

pub struct SplashState {
    asset: String,
    ms: u32,
    next: GameState,
}

pub struct MenuState {
    background: Option<String>,
    title: Option<String>,
    heading: Option<String>,
    items: Vec<MenuItem>,
}

pub enum MenuItem {
    Text(MenuTextItem),
    Layout(MenuLayoutItem),
}

pub struct MenuTextItem {
    text: String,
    id: Option<String>,
    next: GameState,
}

pub enum MenuLayoutItem {
    Break,
}
