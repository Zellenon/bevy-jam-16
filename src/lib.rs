// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod assets;
mod audio;
mod data;
mod demo;
#[cfg(feature = "dev")]
mod dev_tools;
mod gameplay;
mod level;
mod menus;
mod prefabs;
mod screens;
mod theme;
mod utils;

use bevy::{
    asset::AssetMetaCheck,
    input::mouse::{AccumulatedMouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};
use bevy_turborand::prelude::RngPlugin;

pub mod prelude {
    pub use crate::assets::{GameAssets, UiAssets};
    pub use crate::screens::Screen;
    pub use crate::{AppSystems, PausableSystems};
}

pub struct AppPlugin;

const WINDOW_X: f32 = 1280.0;
const WINDOW_Y: f32 = 720.0;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    file_path: "assets/tower_combinator".to_string(),
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Zod's Tower Defender".to_string(),
                        resolution: (WINDOW_X, WINDOW_Y).into(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            data::plugin,
            assets::plugin,
            audio::plugin,
            demo::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            level::plugin,
            menus::plugin,
            prefabs::plugin,
            screens::plugin,
            theme::plugin,
            RngPlugin::default(),
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Camera controls
        app.add_systems(
            Update,
            cameraman
                .run_if(in_state(crate::prelude::Screen::Gameplay))
                .in_set(PausableSystems),
        );
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Projection::Orthographic({
            let mut proj = OrthographicProjection::default_2d();
            proj.scale = 0.05;
            proj
        }),
        Transform::from_translation(Vec3::new(20., 14., 0.)).with_scale(Vec3::new(1.75, 1.75, 1.0)),
    ));
}

fn cameraman(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
) {
    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };

    if mouse_button_input.any_pressed([MouseButton::Right, MouseButton::Middle]) {
        let delta = accumulated_mouse_motion.delta;
        camera_transform.translation += Vec2::new(-delta.x / 50., delta.y / 50.).extend(0.0);
    }

    for mouse_wheel_event in mouse_wheel_events.read() {
        match mouse_wheel_event.unit {
            MouseScrollUnit::Line => {
                // Case 1: Use is using a scroll wheel
                let dy = mouse_wheel_event.y;
                camera_transform.scale -= Vec2::splat(dy / 100.).extend(0.0);
                if camera_transform.scale.x > 2.5 {
                    camera_transform.scale = Vec2::splat(2.5).extend(1.);
                } else if camera_transform.scale.x < 0.5 {
                    camera_transform.scale = Vec2::splat(0.5).extend(1.);
                }
            }
            MouseScrollUnit::Pixel => {
                // This is trackpad-like behavior. Use shift to zoom, else follow
                // trackpad as camera translation
                let dy = mouse_wheel_event.y;
                let dx = mouse_wheel_event.x;

                if keys.pressed(KeyCode::ShiftLeft) {
                    camera_transform.scale -= Vec2::splat(dy / 100.).extend(0.0);
                    if camera_transform.scale.x > 2.5 {
                        camera_transform.scale = Vec2::splat(2.5).extend(1.);
                    } else if camera_transform.scale.x < 0.5 {
                        camera_transform.scale = Vec2::splat(0.5).extend(1.);
                    }
                    // Space was pressed
                } else {
                    camera_transform.translation += Vec2::new(-dx / 50., dy / 50.).extend(0.0);
                }
            }
        }
    }
}
