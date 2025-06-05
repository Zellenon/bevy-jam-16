use crate::data::Tower;
use crate::gameplay::animation::AnimationFrameQueue;
use crate::level::components::LEVEL_SCALING;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct TowerSprites {
    #[asset(path = "images/towers/piston.png")]
    piston_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 6, rows = 1))]
    piston_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/towers/fan.png")]
    fan_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 2, rows = 1))]
    fan_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/towers/spikes.png")]
    spike_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 1, rows = 1))]
    spike_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/towers/oil.png")]
    oil_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 2, rows = 1))]
    oil_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/towers/tesla.png")]
    tesla_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 9, rows = 1))]
    tesla_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/towers/bucket.png")]
    water_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 6, rows = 6))]
    water_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/towers/acid.png")]
    acid_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 3, rows = 1))]
    acid_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/towers/fire.png")]
    flame_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 3, rows = 1))]
    flame_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/towers/portal.png")]
    portal_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 1, rows = 1))]
    portal_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/towers/ice.png")]
    ice_sprite: Handle<Image>,
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 1, rows = 1))]
    ice_layout: Handle<TextureAtlasLayout>,
}

impl TowerSprites {
    pub fn tower_sprite(&self, tower: &Tower) -> (&Handle<Image>, &Handle<TextureAtlasLayout>) {
        match tower {
            Tower::Piston => (&self.piston_sprite, &self.piston_layout),
            Tower::Fan => (&self.fan_sprite, &self.fan_layout),
            Tower::SpikePit => (&self.spike_sprite, &self.spike_layout),
            Tower::Oil => (&self.oil_sprite, &self.oil_layout),
            Tower::TrapDoor => (&self.tesla_sprite, &self.tesla_layout),
            Tower::Tesla => (&self.tesla_sprite, &self.tesla_layout),
            Tower::Water => (&self.water_sprite, &self.water_layout),
            Tower::Acid => (&self.acid_sprite, &self.acid_layout),
            Tower::Flame => (&self.flame_sprite, &self.flame_layout),
            Tower::Portal => (&self.portal_sprite, &self.portal_layout),
            Tower::Ice => (&self.ice_sprite, &self.ice_layout),
        }
    }

    pub fn tower_idle_frames(&self, tower: &Tower) -> &'static [usize] {
        match tower {
            Tower::Piston => &[0, 1, 2, 3, 4, 5, 5, 5],
            Tower::Fan => &[0, 1],
            Tower::SpikePit => &[0],
            Tower::Oil => &[0, 1],
            Tower::TrapDoor => &[0],
            Tower::Tesla => &[0, 1, 2, 3, 4],
            Tower::Water => &[0, 1, 2, 3, 4, 5, 6, 7],
            Tower::Acid => &[0, 1, 2],
            Tower::Flame => &[0, 1, 2],
            Tower::Portal => &[0],
            Tower::Ice => &[0],
        }
    }

    pub fn tower_attack_frames(&self, tower: &Tower) -> &'static [usize] {
        match tower {
            Tower::Tesla => &[5, 6, 7, 8],
            Tower::Water => &[8, 9, 10],
            _ => todo!(),
        }
    }

    pub fn tower_bundle(&self, tower: &Tower) -> impl Bundle {
        let (image, atlas) = self.tower_sprite(tower);

        let mut animation_controller = AnimationFrameQueue::new(self.tower_idle_frames(tower));

        if tower == &Tower::Tesla || tower == &Tower::Water {
            animation_controller.set_override(self.tower_attack_frames(tower));
        }

        (
            Sprite {
                image: image.clone(),
                custom_size: Some(Vec2::splat(LEVEL_SCALING)),
                texture_atlas: Some(TextureAtlas::from(atlas.clone())),
                ..default()
            },
            animation_controller,
        )
    }
}
