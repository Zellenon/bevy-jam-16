use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "audio/sound_effects/button_hover.ogg")]
    pub button_hover_sound: Handle<AudioSource>,
    #[asset(path = "audio/sound_effects/button_click.ogg")]
    pub button_click_sound: Handle<AudioSource>,
    #[asset(path = "ui/hotbar_tesla.png")]
    pub hotbar_tesla_image: Handle<Image>,
    #[asset(path = "ui/hotbar_water.png")]
    pub hotbar_water_image: Handle<Image>,
    #[asset(path = "ui/hotbar_trapdoor.png")]
    pub hotbar_trapdoor_image: Handle<Image>,
}
