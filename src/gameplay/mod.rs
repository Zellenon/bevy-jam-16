use bevy::prelude::*;

mod hotbar;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(hotbar::plugin);
}
