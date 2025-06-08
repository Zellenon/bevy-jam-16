use bevy::{
    ecs::resource::Resource,
    image::TextureAtlasLayout,
    prelude::{Handle, Image, Res, ResMut},
};
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct LevelAssets {
    #[asset(path = "images/floortiles.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub floortiles: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 8, rows = 8))]
    pub floortiles_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/level.png")]
    pub level: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 8, columns = 1, rows = 4))]
    pub level_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/portal.png")]
    pub enemy_spawner: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 5, rows = 1))]
    pub spawner_layout: Handle<TextureAtlasLayout>,
}
