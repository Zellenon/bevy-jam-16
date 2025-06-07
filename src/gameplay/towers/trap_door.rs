use crate::{
    data::Tower,
    gameplay::animation::AnimationFrameQueue,
    level::{components::Adjacent, resource::CellDirection},
    prefabs::physics::GamePhysicsLayer as GPL,
};
use avian2d::prelude::CollisionLayers;
use bevy::{
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        hierarchy::ChildOf,
        system::{Commands, Query, Res, ResMut},
    },
    prelude::Component,
    reflect::Reflect,
    time::{Time, Timer, TimerMode},
};
use bevy_turborand::{DelegatedRng, GlobalRng};

#[derive(Event, Reflect, Debug, PartialEq, Clone, Copy)]
pub struct DetectTrapDoor(pub Entity);

#[derive(Event, Reflect, Debug, PartialEq, Clone, Copy)]
pub struct OpenTrapDoor(pub Adjacent);

#[derive(Component, Reflect, Debug, PartialEq, Clone)]
pub struct TrapDoor {
    close_timer: Timer,
}

pub fn detect_trap_door(
    mut open_trap_door_writer: EventWriter<OpenTrapDoor>,
    mut events: EventReader<DetectTrapDoor>,
    mut towers: Query<(&Tower, &ChildOf, &mut AnimationFrameQueue)>,
    colliders: Query<&Adjacent>,
    mut rng: ResMut<GlobalRng>,
) {
    for DetectTrapDoor(e) in events.read() {
        let Ok((tower, parent, mut animation)) = towers.get_mut(*e) else {
            return;
        };

        let n = 3; // TODO  upgrades will lower this number

        if 0 != rng.usize(0..n) {
            return;
        }

        animation.set_override(CellDirection::Down.attack_frames(&tower));
        if let Ok(desired_id) = colliders.get(parent.0) {
            open_trap_door_writer.write(OpenTrapDoor(*desired_id));
        };
    }
}

pub fn open_trap_door(
    mut events: EventReader<OpenTrapDoor>,
    mut colliders: Query<(Entity, &Adjacent, &mut CollisionLayers)>,
    mut commands: Commands,
) {
    for OpenTrapDoor(floor_entity) in events.read() {
        for (entity, adjacent, mut collisions_layer) in colliders.iter_mut() {
            if adjacent.id == floor_entity.id {
                collisions_layer.filters = [GPL::Level, GPL::Default, GPL::Projectiles].into();
                commands.entity(entity).insert(TrapDoor {
                    close_timer: Timer::from_seconds(1.0, TimerMode::Once),
                });
            }
        }
    }
}

pub fn close_trap_door(
    time: Res<Time>,
    mut colliders: Query<(Entity, &mut TrapDoor, &mut CollisionLayers)>,
    mut commands: Commands,
) {
    for (entity, mut trap_door, mut collisions_layer) in colliders.iter_mut() {
        trap_door.close_timer.tick(time.delta());
        if trap_door.close_timer.just_finished() {
            collisions_layer.filters = [GPL::Enemy, GPL::Default, GPL::Projectiles].into();
            commands.entity(entity).remove::<TrapDoor>();
        }
    }
}
