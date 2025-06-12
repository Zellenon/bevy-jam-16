use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        hierarchy::Children,
        query::With,
        system::{Commands, Query, Res},
    },
    reflect::Reflect,
    sprite::Sprite,
    time::Time,
    transform::components::Transform,
};
use std::marker::PhantomData;

use crate::{
    assets::StatusSprites, data::status_effects::StatusEffectTrait, prefabs::enemies::EnemySprite,
};

use super::common::{ApplyStatus, RemoveStatus};

#[derive(Reflect, Debug, Component, PartialEq, Eq)]
pub struct StatusAnimation<T> {
    #[reflect(ignore)]
    _phantom: PhantomData<T>,
}

pub fn animate_status_effect<T: StatusEffectTrait>(
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut Sprite), With<StatusAnimation<T>>>,
) {
    for (entity, mut sprite) in enemies.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index >= 5 {
                atlas.index = 0
            } else {
                atlas.index += 1
            }
        }
    }
}

pub fn add_status_animation<T: StatusEffectTrait>(
    mut events: EventReader<ApplyStatus<T>>,
    children: Query<&Children>,
    enemy_sprites: Query<(), With<EnemySprite>>,
    existing_status_animations: Query<(), With<StatusAnimation<T>>>,
    sprites: Option<Res<StatusSprites>>,
    mut commands: Commands,
) {
    let sprites = sprites.expect("GameAssets should be available");
    for event in events.read() {
        let e = event.enemy;

        // Get the entity that holds the enemy's sprite
        let enemy_sprite_entity = children
            .get(e)
            .unwrap()
            .iter()
            .filter(|w| enemy_sprites.get(**w).is_ok())
            .next()
            .unwrap();

        // Get rid of any existing Entity in charge of representing this status effect
        if let Ok(children_) = children.get(*enemy_sprite_entity) {
            for entity in children_
                .iter()
                .filter(|w| existing_status_animations.get(**w).is_ok())
            {
                commands.entity(*entity).despawn();
            }
        }

        let status_sprite_bundle = sprites.status_bundle(T::corresponding_enum());

        commands.entity(*enemy_sprite_entity).with_children(|p| {
            p.spawn((
                StatusAnimation::<T>::new(),
                status_sprite_bundle,
                Transform::default(),
            ));
        });
    }
}

pub fn remove_status_animation_on_timeout<T: StatusEffectTrait>(
    mut events: EventReader<RemoveStatus<T>>,
    children: Query<&Children>,
    existing_status_animations: Query<(), With<StatusAnimation<T>>>,
    mut commands: Commands,
) {
    for RemoveStatus {
        enemy, strength, ..
    } in events.read()
    {
        let slated_for_removal = children
            .iter_descendants(*enemy)
            .filter(|w| existing_status_animations.get(*w).is_ok());
        for e in slated_for_removal {
            commands.entity(e).despawn();
        }
    }
}

impl<T: StatusEffectTrait> StatusAnimation<T> {
    pub fn new() -> StatusAnimation<T> {
        StatusAnimation {
            _phantom: PhantomData,
        }
    }
}
