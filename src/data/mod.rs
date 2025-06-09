use crate::prelude::*;
use bevy::prelude::*;
use projectiles::{AttackSpecification, DamageType, Droplet, LiquidType, Puddle};
pub use status_effects::{StatusEffect, StatusEffectTrait};

mod input_state;
pub mod levels;
pub mod projectiles;
mod state;
pub mod stats;
pub mod status_effects;
mod towers;

pub use {
    input_state::PointerInteractionState,
    state::PlayerState,
    towers::get_collision,
    towers::{Tower, TowerCollision},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerState>()
        .insert_resource(levels::LevelData::default());
    app.init_state::<PointerInteractionState>();

    app
        //.register_type::<AttackType>()
        //.register_type::<AttackEffect>()
        .register_type::<Droplet>()
        .register_type::<Puddle>()
        .register_type::<LiquidType>()
        .register_type::<AttackSpecification>()
        .register_type::<DamageType>()
        .register_type::<PlayerState>()
        .register_type::<Tower>()
        .register_type::<PointerInteractionState>();

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
