use crate::prelude::*;
use bevy::prelude::*;

mod input_state;
mod state;
mod towers;
mod status_effects;

pub use {input_state::PointerInteractionState, state::PlayerState, towers::Tower};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerState>();
    app.init_state::<PointerInteractionState>();

    app.add_systems(OnExit(Screen::Loading), validate_assets);
}

fn validate_assets(ui_assets: Res<UiAssets>) {
    for tower in Tower::all() {
        let key = tower.ui_asset_key();
        assert!(
            ui_assets.hotbar_icons.contains_key(key),
            "missing ui asset for {tower:?} ({key})"
        );
    }
}
