use crate::data::projectiles::LiquidType;
use crate::gameplay::animation::AnimationFrameQueue;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct LiquidSprites {
    #[asset(path = "images/liquids/oil.png")]
    oil_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 10, rows = 3))]
    oil_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/liquids/water.png")]
    water_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 10, rows = 3))]
    water_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/liquids/acid.png")]
    acid_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 10, rows = 3))]
    acid_layout: Handle<TextureAtlasLayout>,
}

impl LiquidSprites {
    pub fn droplet(
        &self,
        liquid_type: &LiquidType,
    ) -> (&Handle<Image>, &Handle<TextureAtlasLayout>) {
        match liquid_type {
            LiquidType::Oil => (&self.oil_sprite, &self.oil_layout),
            LiquidType::Water => (&self.water_sprite, &self.water_layout),
            LiquidType::Acid => (&self.acid_sprite, &self.acid_layout),
        }
    }

    pub fn droplet_animation_frames(&self, liquid_type: &LiquidType) -> &'static [usize] {
        match liquid_type {
            LiquidType::Oil => &[16, 17, 18, 19, 20, 21, 22, 23, 24],
            LiquidType::Water => &[16, 17, 18, 19, 20, 21, 22, 23, 24],
            LiquidType::Acid => &[16, 17, 18, 19, 20, 21, 22, 23, 24],
        }
    }

    pub fn droplet_sprite(&self, liquid_type: &LiquidType) -> Sprite {
        let (image, atlas) = self.droplet(liquid_type);
        let first_frame_index = self.droplet_animation_frames(liquid_type)[0];

        Sprite {
            image: image.clone(),
            custom_size: Some(Vec2::new(7., 7.)),
            texture_atlas: Some(TextureAtlas {
                layout: atlas.clone(),
                index: first_frame_index,
            }),
            ..default()
        }
    }

    pub fn droplet_frame_queue(&self, liquid_type: &LiquidType) -> AnimationFrameQueue {
        AnimationFrameQueue::new(self.droplet_animation_frames(liquid_type))
    }

    pub fn puddle(
        &self,
        liquid_type: &LiquidType,
    ) -> (&Handle<Image>, &Handle<TextureAtlasLayout>) {
        match liquid_type {
            LiquidType::Oil => (&self.oil_sprite, &self.oil_layout),
            LiquidType::Water => (&self.water_sprite, &self.water_layout),
            LiquidType::Acid => (&self.acid_sprite, &self.acid_layout),
        }
    }

    pub fn puddle_animation_frames(&self, liquid_type: &LiquidType) -> &'static [usize] {
        match liquid_type {
            LiquidType::Oil => &[5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            LiquidType::Water => &[5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            LiquidType::Acid => &[5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        }
    }

    pub fn stagnant_puddle_animation_frames(&self, liquid_type: &LiquidType) -> &'static [usize] {
        match liquid_type {
            LiquidType::Oil => &[0, 1, 2, 3, 4],
            LiquidType::Water => &[0, 1, 2, 3, 4],
            LiquidType::Acid => &[0, 1, 2, 3, 4],
        }
    }

    pub fn puddle_sprite(&self, liquid_type: &LiquidType) -> Sprite {
        let (image, atlas) = self.puddle(liquid_type);
        let first_animation_frame = self.puddle_animation_frames(liquid_type)[0];
        Sprite {
            image: image.clone(),
            custom_size: Some(Vec2::new(7., 7.)),
            texture_atlas: Some(TextureAtlas {
                layout: atlas.clone(),
                index: first_animation_frame,
            }),
            ..default()
        }
    }

    pub fn puddle_frame_queue(&self, liquid_type: &LiquidType) -> AnimationFrameQueue {
        let mut a = AnimationFrameQueue::new(self.stagnant_puddle_animation_frames(liquid_type));
        a.set_override(self.puddle_animation_frames(liquid_type));
        a
    }

    pub fn bundle(&self, liquid_type: &LiquidType) -> impl Bundle {}
}
