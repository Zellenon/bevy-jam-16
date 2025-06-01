use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
pub use game_assets::GameAssets;
pub use ui_assets::UiAssets;

pub mod game_assets;
pub mod ui_assets;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Splash)
            .load_collection::<GameAssets>()
            .load_collection::<UiAssets>(),
    );
}
