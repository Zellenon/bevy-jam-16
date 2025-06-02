use crate::{AppSystems, PausableSystems};
use avian2d::{math::*, prelude::*};
use bevy::math::NormedVectorSpace;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<MovementAction>()
        .add_systems(
            Update,
            (follow_path, movement, apply_movement_damping)
                .chain()
                .in_set(AppSystems::Update)
                .in_set(PausableSystems),
        )
        .add_systems(
            PreUpdate,
            sleep_physics
                .in_set(AppSystems::Update)
                .in_set(PausableSystems),
        );
}

fn sleep_physics(mut commands: Commands, enemies: Query<Entity, With<Collider>>) {
    for entity in enemies {
        commands.entity(entity).insert(Sleeping);
    }
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    MoveX(Scalar),
    MoveY(Scalar),
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct EnemyController;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The gravitational acceleration used for a character controller.
#[derive(Component)]
#[allow(dead_code)]
pub struct ControllerGravity(Vector);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct EnemyControllerBundle {
    character_controller: EnemyController,
    body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: ControllerGravity,
    movement: MovementBundle,
    waypoint: Waypoint,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
}

#[derive(Component)]
pub struct Waypoint {
    poi: Vec<Vec2>,
    index: usize,
}

impl MovementBundle {
    pub const fn new(acceleration: Scalar, damping: Scalar) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9)
    }
}

impl EnemyControllerBundle {
    pub fn new(collider: Collider, gravity: Vector) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        let zigzag = Waypoint {
            poi: vec![
                Vector::new(-540., -260.),
                Vector::new(540., -260.),
                Vector::new(540., 0.),
                Vector::new(-540., 0.),
                Vector::new(-540., 260.),
                Vector::new(540., 260.),
            ],
            index: 0,
        };

        Self {
            character_controller: EnemyController,
            body: RigidBody::Kinematic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(10.0),
            gravity: ControllerGravity(gravity),
            movement: MovementBundle::default(),
            waypoint: zigzag,
        }
    }

    pub fn with_movement(mut self, acceleration: Scalar, damping: Scalar) -> Self {
        self.movement = MovementBundle::new(acceleration, damping);
        self
    }
}

/// Sends [`MovementAction`] events based on enemy's waypoint direction
fn follow_path(
    mut movement_event_writer: EventWriter<MovementAction>,
    mut enemies: Query<(&Transform, &mut Waypoint), With<EnemyController>>,
) {
    // path instructions to walk around in a circle
    for (enemy_transform, mut enemy_waypoint) in enemies.iter_mut() {
        let x = enemy_transform.translation.x;
        let y = enemy_transform.translation.y;

        let idx = enemy_waypoint.index;
        let heading_towards = enemy_waypoint.poi[idx];

        let arrived_x = if x.distance(heading_towards.x) > 50.0 {
            let direction = if heading_towards.x > x { 1. } else { -1. };

            movement_event_writer.write(MovementAction::MoveX(direction));
            false
        } else {
            true
        };

        let arrived_y = if y.distance(heading_towards.y) > 50.0 {
            let direction = if heading_towards.y > y { 1. } else { -1. };

            movement_event_writer.write(MovementAction::MoveY(direction));
            false
        } else {
            true
        };

        if arrived_x && arrived_y {
            let next_waypoint = enemy_waypoint.index + 1;
            if next_waypoint > enemy_waypoint.poi.len() - 1 {
                enemy_waypoint.index = 0;
            } else {
                enemy_waypoint.index = next_waypoint;
            }
        }
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(&MovementAcceleration, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (movement_acceleration, mut linear_velocity) in &mut controllers {
            match event {
                MovementAction::MoveX(direction) => {
                    linear_velocity.x += *direction * movement_acceleration.0 * delta_time;
                }
                MovementAction::MoveY(direction) => {
                    linear_velocity.y += *direction * movement_acceleration.0 * delta_time;
                }
            }
        }
    }
}

/// Slows down movement in the X direction.
fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
        linear_velocity.y *= damping_factor.0;
    }
}
