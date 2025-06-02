use crate::demo::enemy_movement::{EnemyController, EnemyControllerBundle};
use crate::demo::player::Player;
use crate::prelude::*;
use avian2d::{
    collision::collision_events::OnCollisionStart,
    math::*,
    prelude::{NarrowPhaseSet, *},
};

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Enemy>();

    app.add_systems(
        // Run collision handling after collision detection.
        //
        // NOTE: The collision implementation here is very basic and a bit buggy.
        //       A collide-and-slide algorithm would likely work better.
        PhysicsSchedule,
        kinematic_controller_collisions.in_set(NarrowPhaseSet::Last),
    );

    app.add_observer(|trigger: Trigger<OnCollisionStart, Enemy>|{
        info!(target=?trigger.target(),event=?trigger.event(), "on_add (global)" )
    });
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Enemy;

/// The enemy spawn.
pub fn enemy_spawn_bundle(
    max_speed: f32,
    game_assets: &GameAssets,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    (
        Name::new("Jack"),
        Enemy,
        Sprite {
            image: game_assets.enemysprite.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            ..default()
        },
        EnemyControllerBundle::new(Collider::round_rectangle(32., 32.0, 2.), Vector::splat(0.))
            .with_movement(max_speed, 0.96),
        Transform::from_scale(Vec2::splat(4.0).extend(1.0))
            .with_translation(Vec3::new(-540.0, -260.0, 1.)),
        CollisionEventsEnabled,
    )
}

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system handles collision response for kinematic character controllers
/// by pushing them along their contact normals by the current penetration depth,
/// and applying velocity corrections in order to snap to slopes, slide along walls,
/// and predict collisions using speculative contacts.
#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    mut commands: Commands,
    collisions: Collisions,
    bodies: Query<&RigidBody>,
    collider_rbs: Query<&ColliderOf, Without<Sensor>>,
    mut character_controllers: Query<
        (&mut Position, &mut LinearVelocity),
        (With<RigidBody>, With<Enemy>),
    >,
    time: Res<Time>,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([&ColliderOf { body: rb1 }, &ColliderOf { body: rb2 }]) =
            collider_rbs.get_many([contacts.collider1, contacts.collider2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;

        let character_rb: RigidBody;
        let is_other_dynamic: bool;

        let (mut position, mut linear_velocity) =
            if let Ok(character) = character_controllers.get_mut(rb1) {
                is_first = true;
                character_rb = *bodies.get(rb1).unwrap();
                is_other_dynamic = bodies.get(rb2).is_ok_and(|rb| rb.is_dynamic());
                character
            } else if let Ok(character) = character_controllers.get_mut(rb2) {
                is_first = false;
                character_rb = *bodies.get(rb2).unwrap();
                is_other_dynamic = bodies.get(rb1).is_ok_and(|rb| rb.is_dynamic());
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers.
        if !character_rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.normal
            } else {
                manifold.normal
            };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.points.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            if deepest_penetration > 0.0 {
                // If the slope is climbable, snap the velocity so that the character
                // up and down the surface smoothly.

                // The character is intersecting an unclimbable object, like a wall.
                // We want the character to slide along the surface, similarly to
                // a collide-and-slide algorithm.

                // Don't apply an impulse if the character is moving away from the surface.
                if linear_velocity.dot(normal) > 0.0 {
                    continue;
                }

                // Slide along the surface, rejecting the velocity along the contact normal.
                let impulse = linear_velocity.reject_from_normalized(normal);
                linear_velocity.0 = impulse;
            } else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = linear_velocity.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude =
                    normal_speed - (deepest_penetration / time.delta_secs_f64().adjust_precision());
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.

                // Avoid climbing up walls.
                impulse.y = impulse.y.max(0.0);
                linear_velocity.0 -= impulse;
            }
        }
    }
}
