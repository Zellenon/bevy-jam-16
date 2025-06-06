use bevy::prelude::*;

pub mod animation;
mod damage_numbers;
pub mod hotbar;
pub mod hud;
pub mod level;
pub mod tower_placement;
pub mod towers;
pub mod wave_manager;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        hotbar::plugin,
        level::plugin,
        towers::plugin,
        tower_placement::plugin,
        wave_manager::plugin,
        animation::plugin,
        hud::plugin,
        damage_numbers::plugin,
    ));
}
