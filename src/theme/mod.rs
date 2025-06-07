//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub mod handles;
pub mod interaction;
pub mod palette;
pub mod tooltips;
pub mod widget;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::handles::*;
    pub use super::tooltips::*;
    pub use super::{interaction::InteractionPalette, palette as ui_palette, widget};
}

use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((interaction::plugin, tooltips::plugin));

    load_internal_binary_asset!(
        app,
        handles::TITLE_FONT,
        "../../assets/tower_combinator/fonts/Robotica.ttf",
        // "../../assets/fonts/Kenney Future Narrow.ttf",
        load_font_from_bytes
    );
    load_internal_binary_asset!(
        app,
        handles::LABEL_FONT,
        "../../assets/tower_combinator/fonts/Robotica.ttf",
        load_font_from_bytes
    );
    load_internal_binary_asset!(
        app,
        handles::BODY_FONT,
        "../../assets/tower_combinator/fonts/Roboto-Regular.ttf",
        load_font_from_bytes
    );
}

pub fn load_font_from_bytes(bytes: &[u8], _path: String) -> Font {
    Font::try_from_bytes(bytes.to_vec()).unwrap()
}
