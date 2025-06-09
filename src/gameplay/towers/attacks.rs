use super::{
    common::{TowerFired, TowerTriggerRange},
    directional::FireDirection,
    piston::Shove,
};
use crate::{
    assets::sound_effects::FireSoundEffect,
    data::{
        Tower,
        projectiles::{
            AttackData, AttackSpecification, Droplet, LiquidType, Puddle, TowerAttackType,
        },
        status_effects::damage_multiplier,
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
use avian2d::prelude::{Collisions, LinearVelocity, OnCollisionStart, Sensor};
use bevy::prelude::*;
use bevy_composable::app_impl::ComplexSpawnable;

#[derive(Event, Reflect, Debug, PartialEq, Clone)]
pub struct ApplyAttackData {
    pub target: Entity,
    pub source: Entity,
    pub effect: AttackData,
}

#[derive(Event, Reflect, Debug, PartialEq, Clone)]
pub struct AttackEnemiesInContact(pub Entity, pub Vec<AttackSpecification>);

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
            TowerAttackType::EntireCell(attack_effects) => {
                contact_events.write(AttackEnemiesInContact(
                    children
                        .iter()
                        .filter(|w| ranges.get(*w).is_ok())
                        .next()
                        .unwrap(),
                    attack_effects,
                ));
            }
            TowerAttackType::Contact(attack_effects) => todo!(),
            TowerAttackType::DropsLiquid(liquid_type) => {
                drop_events.write(DropLiquid(event.0, liquid_type));
            }
            TowerAttackType::ModifiesSelf => {
                detect_trap_door_events.write(DetectTrapDoor(event.0));
            }
        }
    }
}

pub fn dispatch_attack_effects(
    mut attackeffect_events: EventReader<ApplyAttackData>,
    mut damage_events: EventWriter<TryDamageToEnemy>,
    mut status_events: EventWriter<TryApplyStatus>,
    mut shoves: EventWriter<Shove>,
) {
    for ApplyAttackData {
        target,
        source,
        effect,
    } in attackeffect_events.read()
    {
        match effect {
            AttackData::Damage {
                dmg_type,
                strength,
                damage,
            } => {
                damage_events.write(TryDamageToEnemy {
                    damage: (*damage as f32 * damage_multiplier(*strength)) as isize,
                    damage_type: *dmg_type,
                    enemy: *target,
                    strength: 1,
                });
            }
            AttackData::Push {
                direction,
                strength,
                force,
            } => {
                shoves.write(Shove(
                    *target,
                    *direction,
                    force * damage_multiplier(*strength),
                ));
            }
            AttackData::Status { status, strength } => {
                status_events.write(TryApplyStatus {
                    status: *status,
                    enemy: *target,
                    strength: *strength,
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
    mut attack_events: EventWriter<ApplyAttackData>,
    collisions: Collisions,
    directions: Query<&FireDirection>,
    parents: Query<&ChildOf, With<TowerTriggerRange>>,
    enemies: Query<(), With<EnemyHealth>>,
    towers: Query<(&Tower, &CellDirection, &AnimationFrameQueue)>,
) {
    for &AttackEnemiesInContact(sensor, ref effects) in events.read() {
        let direction = parents
            .get(sensor)
            .ok()
            .map(|w| directions.get(w.0).ok().map(|w| w.0))
            .flatten()
            .unwrap_or(CellDirection::Up);

        let enemies: Vec<_> = collisions
            .entities_colliding_with(sensor)
            .filter(|w| enemies.get(*w).is_ok())
            .collect();

        for effect in effects {
            for enemy in &enemies {
                attack_events.write(ApplyAttackData {
                    target: *enemy,
                    source: sensor,
                    effect: match effect {
                        AttackSpecification::Damage(damage_type, damage) => AttackData::Damage {
                            dmg_type: *damage_type,
                            strength: 1,
                            damage: *damage,
                        },
                        AttackSpecification::Push(force) => AttackData::Push {
                            direction: direction,
                            strength: 1,
                            force: *force,
                        },
                        AttackSpecification::Status(status_enum) => AttackData::Status {
                            status: *status_enum,
                            strength: 1,
                        },
                    },
                });
            }
        }
    }
}

pub fn animate_towers_on_attack(
    mut fire_events: EventReader<TowerFired>,
    mut towers: Query<(&mut AnimationFrameQueue, &CellDirection, &Tower)>,
) {
    for TowerFired(e) in fire_events.read() {
        let (mut queue, direction, tower) = towers.get_mut(*e).unwrap();
        queue.set_override(direction.attack_frames(&tower));
    }
}

pub fn play_tower_sfx(
    mut fire_events: EventReader<TowerFired>,
    mut sounds: EventWriter<FireSoundEffect>,
    towers: Query<&Tower>,
) {
    for TowerFired(e) in fire_events.read() {
        if let Some(sfx) = towers.get(*e).unwrap().fire_sfx() {
            sounds.write(FireSoundEffect(sfx));
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
