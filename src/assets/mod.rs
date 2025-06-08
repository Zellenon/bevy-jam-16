use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::prelude::*;

pub mod game_assets;
pub mod level_assets;
pub mod liquid_sprites;
pub mod sound_effects;
pub mod status_sprites;
pub mod tower_sprites;
pub mod ui_assets;

pub use {
    game_assets::GameAssets, level_assets::LevelAssets, liquid_sprites::LiquidSprites,
    sound_effects::SoundEffects, status_sprites::StatusSprites, tower_sprites::TowerSprites,
    ui_assets::UiAssets,
};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Splash)
            .load_collection::<GameAssets>()
            .load_collection::<TowerSprites>()
            .load_collection::<UiAssets>()
            .load_collection::<StatusSprites>()
            .load_collection::<LiquidSprites>()
            .load_collection::<LevelAssets>()
            .load_collection::<SoundEffects>(),
    );

    app.add_systems(OnEnter(Screen::Splash), GameAssets::meshes_and_materials);
}
