use crate::PausableSystems;
use crate::prelude::*;
use bevy::color::palettes::css;
use bevy::prelude::IntoScheduleConfigs;
use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_background);
}

fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rng: ResMut<GlobalRng>,
) {
    commands.insert_resource(ClearColor(css::BLACK.into()));

    let dot_mesh = meshes.add(Mesh::from(Circle::new(0.1)));
    let dot_color = materials.add(Color::from(css::WHITE));

    commands
        .spawn((
            Name::new("Background"),
            InheritedVisibility::default(),
            StateScoped(Screen::Gameplay),
            Transform::default(),
        ))
        .with_children(|parent| {
            for _ in 0..1000 {
                parent.spawn((
                    Name::new("Background dot"),
                    Mesh2d(dot_mesh.clone()),
                    MeshMaterial2d(dot_color.clone()),
                    Transform::from_translation(Vec3::new(
                        rng.f32() * 200.0 - 100.0,
                        rng.f32() * 200.0 - 100.0,
                        -100.0,
                    )),
                ));
            }
        });
}
