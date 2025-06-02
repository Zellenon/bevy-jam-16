use crate::data::towers::Tower;
use bevy::prelude::States;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum PointerInteractionState {
    #[default]
    Selecting,
    Placing(Tower),
}
