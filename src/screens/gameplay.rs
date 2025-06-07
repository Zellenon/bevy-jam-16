//! The screen state for the main gameplay.

use crate::data::{PlayerState, PointerInteractionState};
use crate::{gameplay::level::spawn_level, menus::Menu, screens::Screen, Pause};
use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
    app.add_plugins(crate::gameplay::plugin);

    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(in_state(PointerInteractionState::Selecting))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnEnter(Screen::Gameplay), on_game_start);
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn on_game_start(mut commands: Commands) {
    commands.insert_resource(PlayerState::default());
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
