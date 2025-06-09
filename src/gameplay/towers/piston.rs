use crate::{demo::enemy_health::EnemyHealth, level::resource::CellDirection};
use avian2d::prelude::ExternalImpulse;
use bevy::prelude::*;

#[derive(Event, Reflect, Clone, Debug, Copy, PartialEq)]
pub struct Shove(pub Entity, pub CellDirection, pub f32);

pub fn do_shoves(
    mut events: EventReader<Shove>,
    mut enemies: Query<&mut ExternalImpulse, With<EnemyHealth>>,
) {
    for Shove(e, direction, power) in events.read() {
        if let Ok(mut impulse) = enemies.get_mut(*e) {
            **impulse += Into::<Vec2>::into(*direction) * power
        }
    }
}
