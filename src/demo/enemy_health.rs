use crate::{
    AppSystems, PausableSystems,
    assets::{StatusSprites, game_assets::HEALTH_BAR_WIDTH},
    data::{
        PlayerState, StatusEffect, Tower, TowerCollision, get_collision, projectiles::DamageType,
    },
    demo::enemy_movement::MovementDirection,
    gameplay::shared_systems::Lifetime,
};
use avian2d::prelude::{Collider, CollisionLayers, OnCollisionEnd, OnCollisionStart};
use bevy::ecs::relationship::DescendantIter;
use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};
use std::f32::consts::PI;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            active_tower_collision,
            update_health_bars,
            (kill_at_0_health, do_kill_enemies).chain(),
            (try_enemy_damage, do_enemy_damage).chain(),
        )
            .in_set(PausableSystems)
            .in_set(AppSystems::Update),
    );
    app.add_event::<KillEnemy>()
        .add_event::<DoDamageToEnemy>()
        .add_event::<TryDamageToEnemy>();
    app.add_observer(start_collision_damage_event);
    app.add_observer(stop_collision_damage_event);
}

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealth {
    max: isize,
    current: isize,
}

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
/// Gives money when the entity is killed
pub struct Bounty(pub i32);

#[derive(Component, Default, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyChild;

#[derive(Component, Clone, Copy, PartialEq, Reflect)]
pub struct EnemyHealthBar;

#[derive(Event, Debug)]
pub struct RecordInitCollisionDamage {
    tower_entity: Entity,
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct TryDamageToEnemy {
    pub damage: isize,
    pub damage_type: DamageType,
    pub enemy: Entity,
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct DoDamageToEnemy {
    pub damage: isize,
    pub damage_type: DamageType,
    pub enemy: Entity,
}

#[derive(Event, Debug, Clone, Copy, PartialEq, Reflect)]
pub struct KillEnemy(pub Entity);

pub const DAMAGE_VARIANCE: f32 = 0.15;

pub fn kill_at_0_health(
    enemies: Query<(Entity, &EnemyHealth), Without<Lifetime>>,
    mut events: EventWriter<KillEnemy>,
) {
    for (entity, health) in enemies {
        if health.current <= 0 {
            events.write(KillEnemy(entity));
        }
    }
}

pub fn do_kill_enemies(
    bounty: Query<&Bounty>,
    mut events: EventReader<KillEnemy>,
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
) {
    for event in events.read() {
        commands.entity(event.0).insert(Lifetime::new(0.15));
        commands.entity(event.0).remove::<Collider>();
        commands.entity(event.0).remove::<CollisionLayers>();
        if let Ok(bounty) = bounty.get(event.0) {
            player_state.money += bounty.0;
        }
    }
}

pub fn update_health_bars(
    mut events: EventReader<DoDamageToEnemy>,
    mut health_bars: Query<&mut Transform, With<EnemyHealthBar>>,
    children_query: Query<&Children>,
    enemies: Query<&EnemyHealth>,
) {
    for event in events.read() {
        let Some(health_bar_entity) = DescendantIter::new(&children_query, event.enemy)
            .find(|&entity| health_bars.contains(entity))
        else {
            println!("unable to find health bar for enemy {:?}", event.enemy);
            continue;
        };
        let enemy = enemies.get(event.enemy).unwrap();
        let mut health_bar_transform = health_bars.get_mut(health_bar_entity).unwrap();

        health_bar_transform.scale.x = (enemy.current as f32) / (enemy.max as f32);
        health_bar_transform.translation.x =
            -(HEALTH_BAR_WIDTH * ((enemy.max - enemy.current) as f32 / (enemy.max as f32))) / 2.0;
    }
}

fn start_collision_damage_event(
    trigger: Trigger<OnCollisionStart>,
    towers: Query<&Tower>,
    mut commands: Commands,
    mut damage_events: EventWriter<TryDamageToEnemy>,
) {
    let enemy_target = trigger.event().collider;

    if let Ok(tower) = towers.get(trigger.target()) {
        let tower_collision = get_collision(&tower);

        if let Some(collision) = tower_collision {
            // Perform a immediate damage hit
            damage_events.write(TryDamageToEnemy {
                damage: collision.damage,
                damage_type: DamageType::Physical,
                enemy: enemy_target,
            });

            // Add a collision entity that deals damage on a timer while collision is active
            commands.entity(enemy_target).insert(collision);
        }
    };
}

fn stop_collision_damage_event(trigger: Trigger<OnCollisionEnd>, mut commands: Commands) {
    let enemy_target = trigger.event().collider;

    commands.entity(enemy_target).remove::<TowerCollision>();
}

// This is the timer function that deals damage when activeley in collision
fn active_tower_collision(
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut TowerCollision)>,
    mut damage_events: EventWriter<TryDamageToEnemy>,
) {
    for (entity, mut tower_collision) in enemies.iter_mut() {
        tower_collision.iframe.tick(time.delta());
        if tower_collision.iframe.just_finished() {
            damage_events.write(TryDamageToEnemy {
                damage: tower_collision.damage,
                damage_type: DamageType::Physical,
                enemy: entity,
            });
        }
    }
}

// An ailment is triggered on a timer
pub fn try_enemy_damage(
    mut attempts: EventReader<TryDamageToEnemy>,
    mut successes: EventWriter<DoDamageToEnemy>,
    mut rng: ResMut<GlobalRng>,
) {
    for event in attempts.read() {
        let damage: isize = (rng.f32_normalized() * (event.damage as f32) * DAMAGE_VARIANCE)
            as isize
            + event.damage;
        // TODO: I-Frame logic, which is how damage can fail
        // TODO: Elemental resistances and weaknesses from current status effects
        successes.write(DoDamageToEnemy {
            damage,
            damage_type: event.damage_type,
            enemy: event.enemy,
        });
    }
}

pub fn do_enemy_damage(
    mut events: EventReader<DoDamageToEnemy>,
    mut enemies: Query<&mut EnemyHealth>,
) {
    for event in events.read() {
        if let Ok(mut health) = enemies.get_mut(event.enemy) {
            health.current -= event.damage;
            health.current = health.current.clamp(0, 1000);

            debug!("Enemy {:?} has {} health", event.enemy, health.current);
        } else {
            warn!(target=?event.enemy, "Enemy target not found");
            return;
        };
    }
}

impl EnemyHealth {
    pub fn new(health: isize) -> Self {
        Self {
            max: health,
            current: health,
        }
    }
}
