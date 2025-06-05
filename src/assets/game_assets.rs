use bevy::{color::palettes::css::GREEN, prelude::*};
use bevy_asset_loader::prelude::*;

pub const HEALTH_BAR_WIDTH: f32 = 32.0;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "images/ducky.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub ducky: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 32,
        tile_size_y = 32,
        columns = 6,
        rows = 2,
        padding_x = 1,
        padding_y = 1,
        offset_x = 0,
        offset_y = 0
    ))]
    pub ducky_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths(
            "audio/sound_effects/step1.ogg",
            "audio/sound_effects/step2.ogg",
            "audio/sound_effects/step3.ogg",
            "audio/sound_effects/step4.ogg",
        ),
        collection(typed)
    )]
    pub steps: Vec<Handle<AudioSource>>,
    #[asset(path = "audio/music/Fluffing A Duck.ogg")]
    pub music: Handle<AudioSource>,
    #[asset(path = "audio/music/Monkeys Spinning Monkeys.ogg")]
    pub credit_music: Handle<AudioSource>,
    #[asset(path = "images/badguy.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub enemysprite: Handle<Image>,
    #[asset(path = "images/trooper-Sheet.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub trooper: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 48,
        tile_size_y = 48,
        columns = 7,
        rows = 1,
        padding_x = 0,
        padding_y = 0,
        offset_x = 0,
        offset_y = 0
    ))]
    pub trooper_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "images/trooper2.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub trooper2: Handle<Image>,
    #[asset(texture_atlas_layout(
        tile_size_x = 48,
        tile_size_y = 48,
        columns = 8,
        rows = 1,
        padding_x = 0,
        padding_y = 0,
        offset_x = 0,
        offset_y = 0
    ))]
    pub trooper2_layout: Handle<TextureAtlasLayout>,

    health_bar_rect: Handle<Mesh>,
    health_bar_color: Handle<ColorMaterial>,
}

impl GameAssets {
    pub fn badguy(&self) -> Handle<Image> {
        self.enemysprite.clone()
    }

    pub fn badguy_layout(&self) -> Handle<TextureAtlasLayout> {
        self.ducky_layout.clone()
    }

    pub fn trooper(&self) -> Handle<Image> {
        self.trooper.clone()
    }

    pub fn trooper_layout(&self) -> Handle<TextureAtlasLayout> {
        self.trooper_layout.clone()
    }

    pub fn trooper2(&self) -> Handle<Image> {
        self.trooper2.clone()
    }

    pub fn trooper2_layout(&self) -> Handle<TextureAtlasLayout> {
        self.trooper2_layout.clone()
    }

    pub fn ducky(&self) -> Handle<Image> {
        self.ducky.clone()
    }

    pub fn health_bar_mesh(&self) -> Handle<Mesh> {
        self.health_bar_rect.clone()
    }

    pub fn health_color(&self) -> Handle<ColorMaterial> {
        self.health_bar_color.clone()
    }

    pub fn meshes_and_materials(
        mut resources: ResMut<Self>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        resources.health_bar_rect = meshes.add(Rectangle::new(HEALTH_BAR_WIDTH, 3.));

        resources.health_bar_color = materials.add(Color::from(GREEN));
    }
}
