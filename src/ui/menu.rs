use crate::state::{GameState, MenuItem, MenuState, StateConfig, StateDisplay};
use bevy::prelude::*;
use bevy::utils::HashMap;
use color_eyre::owo_colors::style;
use std::clone;
use std::thread::spawn;

#[derive(Resource)]
pub struct Menu {
    pub selected: Option<Entity>,
    pub menu_state: MenuState,
    pub items_lookup: HashMap<String, Entity>,
}

#[derive(Component)]
pub struct MenuComponent;

#[derive(Component)]
pub struct Selected;

pub fn enter(
    mut commands: Commands,
    state_config: Res<StateConfig>,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
) {
    let StateDisplay::Menu(menu_state) = state_config.get(&state.0).unwrap() else {
        panic!("{state:?} is not a Menu: {state_config:?}");
    };
    let mut selected = None;
    let mut items_lookup = HashMap::new();

    if let Some(background) = &menu_state.background {
        let background = asset_server.load(background);

        commands
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::all(Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage::new(background),
                    ..Default::default()
                });
            });
    }

    let logo = menu_state
        .logo
        .as_ref()
        .map(|logo| asset_server.load(&*logo));

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                size: Size::width(Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(30.0)),
                ..default()
            },
            ..default()
        })
        .insert(MenuComponent)
        .with_children(|parent| {
            if let Some(texture) = logo {
                parent.spawn(ImageBundle {
                    image: UiImage {
                        texture,
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }

            for menu_item in &menu_state.items {
                match menu_item {
                    MenuItem::Text(menu_text_item) => {
                        let entity = parent
                            .spawn(
                                TextBundle::from_section(
                                    &menu_text_item.text,
                                    TextStyle {
                                        font: asset_server.load("typefaces/monogram-extended.ttf"),
                                        font_size: 80.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                )
                                .with_style(Style {
                                    padding: UiRect::all(Val::Px(50.0)),
                                    ..default()
                                })
                                .with_background_color(Color::ORANGE_RED),
                            )
                            .id();

                        if selected.is_none() {
                            selected = Some(entity);
                        }

                        if let Some(id) = &menu_text_item.id {
                            items_lookup.insert(id.clone(), entity);
                        }
                    }
                    MenuItem::Layout(_) => {}
                }
            }
        });

    commands.insert_resource(Menu {
        selected,
        menu_state: menu_state.clone(),
        items_lookup,
    });
}

pub fn update_visual_selection(
    menu_items: Res<Menu>,
    mut items: Query<(Entity, &mut BackgroundColor)>,
) {
    for (entity, mut background_color) in items.iter_mut() {
        if Some(entity) == menu_items.selected {
            background_color.0 = Color::ORANGE_RED.into();
        } else {
            background_color.0 = Color::NONE.into();
        }
    }
}

pub fn action_events() {}

pub fn update() {}

pub fn exit() {}
