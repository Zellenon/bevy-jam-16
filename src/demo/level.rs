//! Spawn the main level.

use crate::prelude::*;
use crate::{
    audio::music,
    demo::{enemy::enemy_spawn_bundle, enemy_health::health_bar_spawn, player::player},
    screens::Screen,
};

use crate::demo::player::Player;

use avian2d::prelude::OnCollisionStart;
use bevy::ecs::observer::{self, Observers};
use bevy::ecs::system::entity_command::observe;
use bevy::prelude::*;

pub(super) fn plugin(_app: &mut App) {
    // empty
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            (
                Name::new("Enemy"),
                enemy_spawn_bundle(1650.0, &assets, &mut texture_atlas_layouts,),
                children![health_bar_spawn(meshes, materials)],
            ),
            player(400.0, &assets, &mut texture_atlas_layouts),
            (Name::new("Gameplay Music"), music(assets.music.clone())),
        ],
    ));
}
