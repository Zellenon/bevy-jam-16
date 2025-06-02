use crate::{AppSystems, PausableSystems};
use avian2d::prelude::OnCollisionStart;
use bevy::{color::palettes::basic::*, prelude::*};

use super::enemy_movement::EnemyController;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_health_bar
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );

    app.add_observer(hit_player);
}

// #[derive(Component, Default)]
// struct EnemyHealth(f32);

#[derive(Component)]
pub struct EnemyHealthBar {
    pub mesh_shape: Rectangle,
    pub health: f32, // Value between 0.0 and 1.0
}

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct EnemyHealthBarBundle {
    pub enemy_health_bar: EnemyHealthBar,
}

impl EnemyHealthBarBundle {
    pub fn new(width: f32, height: f32) -> Self {
        let rectangle = Rectangle::new(width, height);

        Self {
            enemy_health_bar: EnemyHealthBar {
                health: 1.0,
                mesh_shape: rectangle,
            },
        }
    }
}

pub fn health_bar_spawn(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> impl Bundle {
    let enemy_health_bar = EnemyHealthBarBundle::new(32., 3.0);

    let mesh = Mesh::from(enemy_health_bar.enemy_health_bar.mesh_shape);
    let mesh_handle = meshes.add(mesh);
    (
        enemy_health_bar,
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        Transform::from_translation(Vec3::new(0., 14., 0.)),
    )
}

fn update_health_bar(time: Res<Time>, mut query: Query<(&mut EnemyHealthBar, &mut Transform)>) {
    for (mut health_bar, mut transform) in query.iter_mut() {
        // Use time to simulate damage for now
        health_bar.health -= time.delta_secs() * 0.01;
        health_bar.health = health_bar.health.clamp(0.0, 1.0);
        transform.scale.x = health_bar.health;
        transform.translation.x =
            -(health_bar.mesh_shape.size().x * (1.0 - health_bar.health)) / 2.0;
    }
}

fn hit_player(
    trigger: Trigger<OnCollisionStart>,
    enemies: Query<Entity, With<EnemyController>>,
    mut commands: Commands,
) {
    let Some(body) = trigger.event().body else {
        return;
    };
    let collider = trigger.event().collider;

    for entity in enemies {
        info!(?entity);
        if body == entity || collider == entity {
            info!(target=?trigger.target(),event=?trigger.event(), "Player touched the enemy" )
        }
    }
}
