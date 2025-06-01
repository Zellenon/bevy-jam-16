//! Spawn the main level.

use crate::prelude::*;
use crate::{audio::music, demo::player::player, screens::Screen};
use bevy::prelude::*;

pub(super) fn plugin(_app: &mut App) {
    // empty
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            player(400.0, &assets, &mut texture_atlas_layouts),
            (Name::new("Gameplay Music"), music(assets.music.clone()))
        ],
    ));
}
