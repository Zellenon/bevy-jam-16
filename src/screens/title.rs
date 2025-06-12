//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::{
    level::{
        resource::LevelSelect,
        START_LEVEL,
    },
    menus::Menu,
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), open_main_menu);
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>, mut level_select: ResMut<LevelSelect>) {
    level_select.0 = START_LEVEL;
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
