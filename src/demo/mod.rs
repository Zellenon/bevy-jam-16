//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use avian2d::{PhysicsPlugins, prelude::PhysicsDebugPlugin};
use bevy::prelude::*;

mod animation;
pub mod enemy;
pub mod enemy_health;
pub mod enemy_movement;
pub mod level;
mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default().with_length_unit(20.0),
        PhysicsDebugPlugin::default(),
        enemy::plugin,
        enemy_movement::plugin,
        enemy_health::plugin,
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
    ));
}
