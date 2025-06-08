use bevy::prelude::*;

pub mod animation;
mod background;
mod damage_numbers;
pub mod hotbar;
pub mod hud;
pub mod level;
pub mod messages;
pub mod shared_systems;
pub mod stats;
pub mod status_effects;
pub mod tower_placement;
pub mod towers;
pub mod wave_manager;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        damage_numbers::plugin,
        hotbar::plugin,
        hud::plugin,
        level::plugin,
        shared_systems::plugin,
        status_effects::plugin,
        stats::plugin,
        towers::plugin,
        tower_placement::plugin,
        wave_manager::plugin,
        messages::plugin,
        background::plugin,
    ));
}
