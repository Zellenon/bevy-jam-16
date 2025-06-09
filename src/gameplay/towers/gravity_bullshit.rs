use super::common::TowerTriggerNeedsGravity;
use crate::prefabs::physics::GamePhysicsLayer;
use avian2d::prelude::{Collider, LayerMask, RayCaster, RayHits, ScalableCollider};
use bevy::{ecs::entity::EntityHashSet, prelude::*};

#[derive(Clone, Copy, Debug, Reflect, Component, PartialEq, Eq)]
pub struct RangeDropper(pub Entity);

pub fn spawn_rangedroppers(
    ranges: Query<(Entity, &GlobalTransform), With<TowerTriggerNeedsGravity>>,
    mut commands: Commands,
) {
    let filter: LayerMask = [GamePhysicsLayer::Level].into();

    for (e, transform) in ranges.iter() {
        let (_, rotation, location) = transform.to_scale_rotation_translation();
        let raycaster = RayCaster::new(Vec2::ZERO, Dir2::SOUTH).with_query_filter(
            avian2d::prelude::SpatialQueryFilter {
                mask: [GamePhysicsLayer::Level].into(),
                excluded_entities: EntityHashSet::new(),
            },
        );
        commands.spawn((
            Transform::from_translation(location),
            RangeDropper(e),
            raycaster,
        ));
        commands.entity(e).remove::<TowerTriggerNeedsGravity>();
    }
}

pub fn drop_ranges(
    droppers: Query<(Entity, &RayHits, &RangeDropper)>,
    mut ranges: Query<&mut Transform>,
    mut commands: Commands,
) {
    for (dropper, hits, RangeDropper(target_entity)) in droppers.iter() {
        if let Ok(mut pos) = ranges.get_mut(*target_entity) {
            let mut ray_iter = hits.iter_sorted();
            ray_iter.next();
            let floor = ray_iter.next();
            if let Some(hit_data) = floor {
                let distance = hit_data.distance; // 14.5
                let collider_height = distance + 5.; // 20.
                pos.translation -= Vec3::new(0., collider_height / 2. - 5., 0.);
                commands
                    .entity(*target_entity)
                    .insert(Collider::rectangle(10., collider_height));
                //collider.scale_by(Vec2::new(1., collider_height / 10.), 0);
                println!("Distance: {}", distance);
                commands.entity(dropper).despawn();
            }
        }
    }
}
