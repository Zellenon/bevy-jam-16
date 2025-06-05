use bevy::prelude::*;

use crate::prelude::*;

mod input_state;
mod state;
mod status_effects;
mod towers;

pub use {
    input_state::PointerInteractionState,
    state::PlayerState,
    status_effects::Ailments,
    status_effects::StatusEffect,
    status_effects::get_ailment,
    towers::{Tower, TowerCollision, get_collison},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerState>();
    app.init_state::<PointerInteractionState>();

    app.add_systems(OnExit(Screen::Loading), validate_assets);
}

fn validate_assets(ui_assets: Res<UiAssets>) {
    for tower in Tower::all() {
        let ui_key = tower.ui_asset_key();
        assert!(
            ui_assets.hotbar_icons.contains_key(ui_key),
            "missing ui asset for {tower:?} ({ui_key})"
        );
    }
}
