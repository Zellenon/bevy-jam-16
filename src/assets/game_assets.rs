use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "images/ducky.png")]
    #[asset(image(sampler(filter = nearest)))]
    pub ducky: Handle<Image>,
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
}
