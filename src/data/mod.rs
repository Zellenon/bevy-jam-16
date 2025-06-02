use bevy::app::App;
use bevy::prelude::AppExtStates;

mod input_state;
mod state;
mod towers;

pub use {
    towers::Tower,
    state::PlayerState,
    input_state::PointerInteractionState
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<state::PlayerState>();
    app.init_state::<input_state::PointerInteractionState>();
}
