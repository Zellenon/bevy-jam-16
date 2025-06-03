//! Spawn the main level.

use crate::{
    audio::music,
    demo::player::player,
    level::{
        components::{LevelParent, pos},
        resource::{Level, MAP_TEXT},
    },
    prefabs::enemies::{basic_trooper, chonkus_trooper, turbo_trooper},
    prelude::*,
    screens::Screen,
};

use bevy::prelude::*;
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    wrappers::name,
};

pub(super) fn plugin(_app: &mut App) {
    // empty
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut level: ResMut<Level>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            //enemy_spawn_bundle(30.0, &assets, &mut texture_atlas_layouts),
            // player(40.0, &assets, &mut texture_atlas_layouts),
            (Name::new("Gameplay Music"), music(assets.music.clone())),
        ],
    ));
    *level = Level::from_str(MAP_TEXT);
    commands.compose(
        LevelParent::from_data(&level)
            + name("Level Parent")
            + StateScoped(Screen::Gameplay).store(),
    );
    commands.compose(basic_trooper() + pos(0., 0.));
    commands.compose(chonkus_trooper() + pos(0., 0.));
    commands.compose(turbo_trooper() + pos(0., 0.));
}
