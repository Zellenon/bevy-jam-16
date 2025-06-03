use bevy::prelude::*;

mod hotbar;
mod turret_placement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((hotbar::plugin, turret_placement::plugin));
}
