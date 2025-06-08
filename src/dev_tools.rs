//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::debug_render::PhysicsGizmos;
use avian2d::prelude::PhysicsDebugPlugin;
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::data::PlayerState;
use crate::{data::PointerInteractionState, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);
    app.add_systems(Update, log_transitions::<PointerInteractionState>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    app.add_systems(Update, add_1k.run_if(input_just_pressed(KeyCode::KeyM)));
    app.add_systems(Startup, on_startup);

    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    });
    app.add_plugins(WorldInspectorPlugin::new());
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn on_startup(mut store: ResMut<GizmoConfigStore>) {
    store.config_mut::<PhysicsGizmos>().0.enabled = false
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>, mut store: ResMut<GizmoConfigStore>) {
    options.toggle();
    let config = store.config_mut::<PhysicsGizmos>().0;
    config.enabled = options.enabled;
}

fn add_1k(mut player_state: ResMut<PlayerState>) {
    player_state.money += 1000;
    info!("Added $1000 to player state.");
}
