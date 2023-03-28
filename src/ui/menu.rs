use crate::input::{Action, ActionEvent, EventState, InputState};
use crate::state::{
    GameState, MenuItem, MenuItemDetails, MenuItemId, MenuState, StateConfig, StateDisplay,
};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::utils::petgraph::visit::Walker;
use bevy::utils::HashMap;
use color_eyre::owo_colors::style;
use std::clone;
use std::process::id;
use std::thread::spawn;

#[derive(Resource)]
pub struct Menu {
    pub selected: Option<MenuItemId>,
    pub menu_state: MenuState,
    pub items_lookup: HashMap<MenuItemId, Entity>,
}

impl Menu {
    pub fn select_next(&mut self, direction: i8) {
        let Some(selected) = &self.selected else {
            warn!("No item selected.");
            return;
        };

        let Some(index) = self.menu_state.items.iter().position(|item| item.id.as_ref() == Some(selected)) else {
            warn!("Selected item not found in menu.");
            return;
        };
        let index = index as i32;

        let mut next_index: i32 = index as i32;
        loop {
            next_index += direction as i32;

            if next_index >= self.menu_state.items.len() as i32 {
                next_index = 0;
            } else if next_index < 0 {
                next_index = self.menu_state.items.len() as i32 - 1;
            }

            if next_index == index {
                warn!("No other selectable items found.");
                break;
            }

            let item = &self.menu_state.items[next_index as usize];
            if !item.selectable {
                continue;
            }

            let Some(id) = &item.id else {
                continue;
            };

            self.selected = Some(id.clone());
        }
    }
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
                let maybe_entity = match &menu_item.details {
                    MenuItemDetails::Text(menu_text_item) => {
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
                        Some(entity)
                    }
                    MenuItemDetails::Layout(_) => {
                        warn!("Layouts are not yet supported in menus.");
                        None
                    }
                };

                if let Some(id) = &menu_item.id {
                    if selected.is_none() {
                        selected = Some(id.clone());
                    }
                    let Some(entity) = maybe_entity else {
                        continue;
                    };

                    items_lookup.insert(id.clone(), entity);
                }
            }
        });

    commands.insert_resource(Menu {
        selected,
        menu_state: menu_state.clone(),
        items_lookup,
    });
}

pub fn update_visual_selection(menu: Res<Menu>, mut items: Query<(Entity, &mut BackgroundColor)>) {
    let Some(selected_id) = &menu.selected else {
        warn!("Can't change selection, there is no selection.");
        return;
    };

    let Some(selected_entity) = menu.items_lookup.get(selected_id) else {
        warn!("Can't change selection, no menu item with ID {selected_id:?} found.");
        return;
    };

    for (entity, mut background_color) in items.iter_mut() {
        if entity == *selected_entity {
            background_color.0 = Color::ORANGE_RED.into();
        } else {
            background_color.0 = Color::NONE.into();
        }
    }
}

pub fn handle_action_events(
    mut menu: ResMut<Menu>,
    mut action_event_reader: EventReader<ActionEvent>,
) {
    for event in action_event_reader.iter() {
        // We only care about just_pressed state.
        if event.state != EventState::Released {
            continue;
        }

        match event.action {
            Action::Down => {
                menu.select_next(1);
            }
            Action::Up => {
                menu.select_next(-1);
            }
            _ => {}
        }
    }
}

pub fn update() {}

pub fn exit() {}
