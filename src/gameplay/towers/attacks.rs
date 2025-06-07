use avian2d::prelude::{Collisions, LinearVelocity, OnCollisionStart, Sensor};
use bevy::{
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        hierarchy::Children,
        observer::Trigger,
        query::With,
        system::{Commands, Query},
    },
    math::{Vec2, Vec3Swizzles},
    prelude::warn,
    reflect::Reflect,
    transform::components::{GlobalTransform, Transform},
};
use bevy_composable::app_impl::ComplexSpawnable;
use std::cell;

use super::common::{TowerFired, TowerTriggerRange};
use crate::{
    assets::LiquidSprites,
    data::{
        Tower,
        projectiles::{AttackEffect, AttackType, DamageType, Droplet, LiquidType, Puddle},
    },
    demo::enemy_health::{EnemyHealth, TryDamageToEnemy},
    gameplay::{
        animation::AnimationFrameQueue, status_effects::common::TryApplyStatus,
        towers::trap_door::DetectTrapDoor,
    },
    level::{
        components::{Architecture, pos},
        resource::CellDirection,
    },
    prefabs::attacks::{droplet, puddle},
};

#[derive(Event, Reflect, Debug, PartialEq, Clone)]
pub struct ApplyAttackEffect {
    pub target: Entity,
    pub source: Entity,
    pub effect: AttackEffect,
}

#[derive(Event, Reflect, Debug, PartialEq, Clone)]
pub struct AttackEnemiesInContact {
    pub sensor: Entity,
    pub effect: Vec<AttackEffect>,
    pub tower: Entity,
}

#[derive(Event, Reflect, Debug, PartialEq, Clone, Copy)]
pub struct DropLiquid(pub Entity, pub LiquidType);

pub fn do_tower_attacks(
    mut fire_events: EventReader<TowerFired>,
    mut contact_events: EventWriter<AttackEnemiesInContact>,
    mut drop_events: EventWriter<DropLiquid>,
    mut detect_trap_door_events: EventWriter<DetectTrapDoor>,
    towers: Query<(&Tower, &Children, &GlobalTransform)>,
    ranges: Query<(), With<TowerTriggerRange>>,
) {
    for event in fire_events.read() {
        let Ok((tower, children, global_pos)) = towers.get(event.0) else {
            warn!("Tower not found in dispatch_attack_effects");
            return;
        };

        match tower.attack_def() {
            AttackType::EntireCell(attack_effects) => {
                contact_events.write(AttackEnemiesInContact {
                    sensor: *children
                        .iter()
                        .filter(|w| ranges.get(**w).is_ok())
                        .next()
                        .unwrap(),
                    effect: attack_effects,
                    tower: event.0,
                });
            }
            AttackType::Contact(attack_effects) => todo!(),
            AttackType::DropsLiquid(liquid_type) => {
                bevy::prelude::info!(entity = ?event.0, liquid_type = ?liquid_type, "here i am");
                drop_events.write(DropLiquid(event.0, liquid_type));
            }
            AttackType::ModifiesSelf => {
                detect_trap_door_events.write(DetectTrapDoor(event.0));
            }
        }
    }
}

pub fn dispatch_attack_effects(
    mut attackeffect_events: EventReader<ApplyAttackEffect>,
    mut damage_events: EventWriter<TryDamageToEnemy>,
    mut status_events: EventWriter<TryApplyStatus>,
) {
    for (ApplyAttackEffect {
        target,
        source,
        effect,
    }) in attackeffect_events.read()
    {
        match effect {
            AttackEffect::Damage(damage_type) => {
                damage_events.write(TryDamageToEnemy {
                    damage: 10, // TODO: Add damage details
                    damage_type: *damage_type,
                    enemy: *target,
                });
            }
            AttackEffect::Push => todo!(),
            AttackEffect::Status(status_effect) => {
                status_events.write(TryApplyStatus {
                    status: *status_effect,
                    enemy: *target,
                    strength: 1, // TODO: Update with strength system
                });
            }
        }
    }
}

pub fn splat_droplets(
    trigger: Trigger<OnCollisionStart>,
    sensors: Query<(), With<Sensor>>,
    droplets: Query<(&Transform, &Droplet)>,
    mut commands: Commands,
) {
    let droplet = trigger.target();
    let other = trigger.collider;

    // We don't want droplets to do things when they hit sensors
    if sensors.get(other).is_err() {
        if let Ok((transform, Droplet(liquid))) = droplets.get(droplet) {
            let loc = transform.translation;
            commands.entity(droplet).despawn();
            commands.compose(puddle(*liquid) + pos(loc.x, loc.y));
        }
    }
}

pub fn attack_contact_enemies(
    mut events: EventReader<AttackEnemiesInContact>,
    mut attack_events: EventWriter<ApplyAttackEffect>,
    collisions: Collisions,
    enemies: Query<(), With<EnemyHealth>>,
    mut towers: Query<(&Tower, &CellDirection, &mut AnimationFrameQueue)>,
) {
    for &AttackEnemiesInContact {
        sensor,
        ref effect,
        tower,
    } in events.read()
    {
        let enemies: Vec<_> = collisions
            .entities_colliding_with(sensor)
            .filter(|w| enemies.get(*w).is_ok())
            .collect();
        let Ok((tower, cell_direction, mut animation)) = towers.get_mut(tower) else {
            warn!("Tower not found in dispatch_attack_effects");
            return;
        };
        animation.set_override(cell_direction.attack_frames(&tower));
        for effect in effect {
            for enemy in &enemies {
                attack_events.write(ApplyAttackEffect {
                    target: *enemy,
                    source: sensor,
                    effect: effect.clone(),
                });
            }
        }
    }
}

pub fn stop_dropping_puddles(
    trigger: Trigger<OnCollisionStart>,
    level_parts: Query<(), With<Architecture>>,
    mut droplets: Query<&mut LinearVelocity, With<Puddle>>,
) {
    let puddle = trigger.target();
    let other = trigger.collider;

    if level_parts.get(other).is_ok() {
        if let Ok(mut vel) = droplets.get_mut(puddle) {
            vel.0 = Vec2::ZERO;
        }
    }
}
