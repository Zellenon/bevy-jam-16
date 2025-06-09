use super::directional::FireDirection;
use crate::{
    demo::enemy_health::EnemyHealth,
    level::{components::PathNode, resource::CellDirection},
    prefabs::physics::GamePhysicsLayer,
};
use avian2d::prelude::{
    Collider, Collisions, ExternalImpulse, LayerMask, RayCaster, RayHits, Sensor,
};
use bevy::{ecs::entity::EntityHashSet, prelude::*};

#[derive(Component, Reflect, Debug, Clone, Copy)]
pub struct ForceField(pub CellDirection);

#[derive(Component, Reflect, Debug, Clone, Copy)]
pub struct FanNeedsDirection;

#[derive(Component, Reflect, Debug, Copy, Clone)]
pub struct FanCaster(Entity);

pub fn spawn_fancasters(
    fans: Query<(Entity, &GlobalTransform), With<FanNeedsDirection>>,
    nodes: Query<(&Transform, &PathNode)>,
    mut commands: Commands,
) {
    let filter: LayerMask = [GamePhysicsLayer::Level].into();

    for (e, transform) in fans.iter() {
        let (_, rotation, location) = transform.to_scale_rotation_translation();

        let mut nodes_sorted_by_distance = nodes
            .iter()
            .map(|w| {
                (
                    location.distance(w.0.translation.xy().extend(0.)),
                    (w.0, w.1.prev_direction),
                )
            })
            .collect::<Vec<_>>();
        nodes_sorted_by_distance.sort_by(|w, other| w.0.total_cmp(&other.0));
        let direction = nodes_sorted_by_distance[0].1.1.flip();

        let raycaster = RayCaster::new(Vec2::ZERO, Dir2::new_unchecked(direction.into()))
            .with_query_filter(avian2d::prelude::SpatialQueryFilter {
                mask: [GamePhysicsLayer::Level].into(),
                excluded_entities: EntityHashSet::new(),
            });
        commands.spawn((
            Transform::from_translation(location),
            FanCaster(e),
            raycaster,
        ));

        commands
            .entity(e)
            .insert(FireDirection(direction))
            .remove::<FanNeedsDirection>();
    }
}

pub fn resolve_fancasters(
    casters: Query<(Entity, &RayHits, &FanCaster)>,
    fans: Query<&FireDirection>,
    mut commands: Commands,
) {
    for (dropper, hits, FanCaster(fan_entity)) in casters.iter() {
        let mut ray_iter = hits.iter_sorted();
        ray_iter.next();
        let surface = ray_iter.next();
        if let Some(hit_data) = surface {
            let distance = hit_data.distance;
            let collider_size = distance + 5.;
            let direction = fans.get(*fan_entity).unwrap().0;
            let direction_vec: Vec2 = direction.into();
            let translation = direction_vec.extend(0.) * (collider_size / 2. - 5.);
            let collider = match direction {
                CellDirection::Up | CellDirection::Down => Collider::rectangle(9., collider_size),
                CellDirection::Left | CellDirection::Right => {
                    Collider::rectangle(collider_size, 9.)
                }
            };
            commands.entity(*fan_entity).with_children(|w| {
                w.spawn((
                    ForceField(direction),
                    collider,
                    Sensor,
                    Transform::from_translation(translation),
                ));
            });
            println!("Distance: {}", distance);
            commands.entity(dropper).despawn();
        }
    }
}

pub fn do_forcefields(
    collisions: Collisions,
    fields: Query<(Entity, &ForceField)>,
    mut enemies: Query<&mut ExternalImpulse, With<EnemyHealth>>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    for (field_e, ForceField(direction)) in fields.iter() {
        let direction: Vec2 = (*direction).into();
        let pushed_enemies: Vec<_> = collisions
            .entities_colliding_with(field_e)
            .filter(|w| enemies.get(*w).is_ok())
            .collect();

        for enemy in pushed_enemies {
            if let Ok(mut enemy) = enemies.get_mut(enemy) {
                **enemy += direction * 45. * delta;
            }
        }
    }
}
