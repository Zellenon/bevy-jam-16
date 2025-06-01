use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "audio/sound_effects/button_hover.ogg")]
    pub button_hover_sound: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/button_click.ogg")]
    pub button_click_sound: Handle<AudioSource>,
}
