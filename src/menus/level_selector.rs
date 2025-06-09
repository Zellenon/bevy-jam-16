//! The credits menu.

use crate::data::levels::LevelData;
use crate::level::resource::{LevelSelect, UnlockedLevels};
use crate::prelude::*;
use crate::{menus::Menu, theme::prelude::*};
use bevy::{
    ecs::spawn::SpawnIter, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*,
};

#[derive(Component)]
struct LevelIndex(usize);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::LevelSelector), spawn_menu);
    app.add_observer(level_select_observer);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::LevelSelector).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_menu(
    mut commands: Commands,
    level_data: Res<LevelData>,
    unlocked_levels: Res<UnlockedLevels>,
) {
    commands.spawn((
        widget::ui_root("Select a Level"),
        GlobalZIndex(2),
        StateScoped(Menu::LevelSelector),
        children![
            level_list(level_data, unlocked_levels),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn level_list(level_data: Res<LevelData>, unlocked_levels: Res<UnlockedLevels>) -> impl Bundle {
    let levels = level_data
        .maps
        .iter()
        .enumerate()
        .map(|(i, _)| {
            info!(?unlocked_levels, i);
            if unlocked_levels.0.contains(&i) {
                (format!("{}", i + 1), i)
            } else {
                (format!("{} (locked)", i + 1), i)
            }
        })
        .collect::<Vec<_>>();

    grid(levels)
}

fn grid(content: Vec<(String, usize)>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().map(|(text, idx)| {
            widget::global_observer_button(text, LevelIndex(idx))
        }))),
    )
}

fn go_back_on_click(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn level_select_observer(
    trigger: Trigger<Pointer<Released>>,
    mut level_select: ResMut<LevelSelect>,
    unlocked_levels: Res<UnlockedLevels>,
    level_index: Query<&LevelIndex>,
    mut next_sceeen: ResMut<NextState<Screen>>,
) {
    if let Ok(level_index) = level_index.get(trigger.target) {
        if unlocked_levels.0.contains(&level_index.0) {
            level_select.0 = level_index.0;
            next_sceeen.set(Screen::LevelTransition);
        }
    };
}
