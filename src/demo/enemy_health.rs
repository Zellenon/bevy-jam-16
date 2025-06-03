use avian2d::prelude::{CollisionEventsEnabled, OnCollisionStart};
use bevy::prelude::*;
use rand::Rng;

use super::enemy_movement::EnemyController;
use crate::assets::game_assets::HEALTH_BAR_WIDTH;
use crate::data::Tower;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(hit_enemy);
    app.add_observer(update_health);
}

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealth(pub f32);

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealthBar;

#[derive(Event, Debug)]
pub struct Attack {
    tower_entity: Entity,
    enemy_health_bar_entity: Entity,
}

impl EnemyHealth {
    pub fn new() -> Self {
        Self(1.0)
    }
}

pub fn update_health(
    trigger: Trigger<Attack>,
    mut bars: Query<(Entity, &ChildOf, &mut Transform), With<EnemyHealthBar>>,
    towers: Query<&Tower>,
    mut enemies: Query<&mut EnemyHealth>,
) {
    let Ok(mut parent_health) = enemies.get_mut(trigger.target()) else {
        warn!(target=?trigger.target(), "Enemy target not found");
        return;
    };

    let Ok(tower) = towers.get(trigger.tower_entity) else {
        warn!(target=?trigger.target(), "Tower not found");
        return;
    };

    let Ok((_, _, mut transform)) = bars.get_mut(trigger.enemy_health_bar_entity) else {
        return;
    };

    let tower_damage = tower.collision_damage();
    let rng = &mut rand::rng();
    let damage = rng.random_range(tower_damage.min..=tower_damage.max);

    parent_health.0 -= damage;
    parent_health.0 = parent_health.0.clamp(0.0, 1.0);
    transform.scale.x = parent_health.0;
    transform.translation.x = -(HEALTH_BAR_WIDTH * (1.0 - parent_health.0)) / 2.0;
}

fn hit_enemy(
    trigger: Trigger<OnCollisionStart>,
    world: bevy::ecs::world::DeferredWorld,
    mut commands: Commands,
) {
    let enemy_target = trigger.event().collider;

    // The health_bar is 2 children deep. Use first child to get the next child
    // This logic is fragile and will break if the prefab changes in the future.
    let Some(children) = world.get::<Children>(enemy_target) else {
        return;
    };

    let Some(child_entity) = children.first().cloned() else {
        return;
    };

    let Some(children) = world.get::<Children>(child_entity) else {
        return;
    };

    let Some(health_bar_entity) = children.first().cloned() else {
        return;
    };

    commands.trigger_targets(
        Attack {
            tower_entity: trigger.target(),
            enemy_health_bar_entity: health_bar_entity,
        },
        enemy_target,
    );
}
