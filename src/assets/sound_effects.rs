use crate::data::Tower;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct SoundEffects {
    #[asset(path = "audio/sound_effects/tower_place.wav")]
    pub tower_placed_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/enemy_spawn.wav")]
    pub enemy_spawn_sfx: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/took_damage.wav")]
    pub took_damage: Handle<AudioSource>,

    #[asset(path = "audio/towers/tesla.ogg")]
    tesla_attack: Handle<AudioSource>,
    #[asset(path = "audio/towers/spike_pit.ogg")]
    spike_pit: Handle<AudioSource>,
}

impl SoundEffects {
    pub fn tower_attack(&self, tower: &Tower) -> Option<Handle<AudioSource>> {
        match tower {
            Tower::Tesla => Some(self.tesla_attack.clone()),
            Tower::SpikePit => Some(self.spike_pit.clone()),
            _ => None,
        }
    }
}
