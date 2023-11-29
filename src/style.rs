//! Styling for UI via bevy_ui_dsl

use bevy::prelude::*;

pub fn node_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.)
}

pub fn node_menu(b: &mut NodeBundle) {
    let s = &mut b.style;
    s.flex_direction = FlexDirection::Column;
    s.justify_content = JustifyContent::Center;
    s.align_items = AlignItems::Center;
    s.padding = UiRect::all(Val::Px(10.));

    node_dark_half_alpha_background(b);
}

pub fn node_dark_half_alpha_background(b: &mut NodeBundle) {
    b.background_color = Color::rgba(0., 0., 0., 0.7).into();
}

pub fn text_bundle(_a: &AssetServer, b: &mut TextBundle) {
    b.style.flex_grow = 0.;
    b.style.margin = UiRect::all(Val::Px(10.));
}

pub fn text_style(assets: &AssetServer, s: &mut TextStyle) {
    s.font_size = 30.;
    s.color = Color::WHITE.into();
}

pub fn button(assets: &AssetServer, b: &mut ButtonBundle) {
    b.style.width = Val::Px(300.);
    b.style.height = Val::Px(100.);
    b.style.align_items = AlignItems::Center;
    b.style.justify_content = JustifyContent::Center;
    b.background_color = Color::PURPLE.into();
}

pub fn button_text(assets: &AssetServer, b: &mut TextBundle) {
    b.style.align_self = AlignSelf::Center;
    b.background_color = Color::PINK.into();
}
