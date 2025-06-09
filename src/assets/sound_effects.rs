use std::sync::Arc;

use crate::audio::sound_effect;
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
    #[asset(path = "audio/towers/piston.ogg")]
    piston: Handle<AudioSource>,
    #[asset(path = "audio/towers/oil.ogg")]
    oil: Handle<AudioSource>,
    #[asset(path = "audio/towers/water.wav")]
    water: Handle<AudioSource>,
}

impl SoundEffects {
    pub fn tower_placed(&self) -> Handle<AudioSource> {
        self.tower_placed_sfx.clone()
    }

    pub fn enemy_spawn(&self) -> Handle<AudioSource> {
        self.enemy_spawn_sfx.clone()
    }

    pub fn damage(&self) -> Handle<AudioSource> {
        self.took_damage.clone()
    }

    pub fn tesla_fire(&self) -> Handle<AudioSource> {
        self.tesla_attack.clone()
    }
    pub fn spike_fire(&self) -> Handle<AudioSource> {
        self.spike_pit.clone()
    }
    pub fn piston_fire(&self) -> Handle<AudioSource> {
        self.piston.clone()
    }
    pub fn oil_fire(&self) -> Handle<AudioSource> {
        self.oil.clone()
    }
    pub fn water_fire(&self) -> Handle<AudioSource> {
        self.water.clone()
    }
}

#[derive(Event, Reflect, Clone)]
pub struct FireSoundEffect(pub Arc<dyn SoundFn>);

pub trait SoundFn: 'static + Sync + Send + Fn(&SoundEffects) -> Handle<AudioSource> {}
impl<F> SoundFn for F where F: Fn(&SoundEffects) -> Handle<AudioSource> + Send + Sync + 'static {}

pub fn fire_sounds(
    mut commands: Commands,
    mut events: EventReader<FireSoundEffect>,
    sfx: Res<SoundEffects>,
) {
    for FireSoundEffect(fetcher) in events.read() {
        commands.spawn(sound_effect(fetcher(&*sfx)));
    }
}
