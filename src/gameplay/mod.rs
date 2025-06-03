use bevy::prelude::*;

mod hotbar;
mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(hotbar::plugin);
}
