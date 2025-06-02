use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerState {
    pub money: i32,
    pub health: i32,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState {
            money: 100,
            health: 100,
        }
    }
}
