use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    prelude::*,
    state::{condition::in_state, state::OnEnter},
    time::{Time, Timer},
    transform::components::Transform,
};
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    tree::ComponentTree,
};
use std::{collections::VecDeque, time::Duration};

use crate::assets::SoundEffects;
use crate::audio::sound_effect;
use crate::{
    PausableSystems,
    assets::UiAssets,
    level::components::StartNode,
    prefabs::enemies::{basic_trooper, chonkus_trooper, turbo_trooper},
    prelude::*,
    theme::widget,
};

#[derive(Resource, Clone)]
pub struct WaveManager {
    current_wave: Option<Wave>,
    upcoming_waves: VecDeque<Wave>,
    wave_timer: Timer,
}

#[derive(Clone, Component)]
pub struct Wave(pub VecDeque<(Group, Duration)>);

// Enemies don't spawn all at once in a wave, they spawn in delayed groups.
#[derive(Clone, Component)]
pub struct Group(pub Vec<ComponentTree>);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(WaveManager::default());
    app.add_systems(OnEnter(Screen::Gameplay), (add_waves, add_spawn_button));
    app.add_systems(
        Update,
        (tick_wave_timer, spawn_next_wave)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_observer(spawn_enter_observer);
    app.add_observer(spawn_leave_observer);
    app.add_observer(spawn_pressed_observer);
    app.add_observer(spawn_released_observer);
}

pub fn add_spawn_button(mut commands: Commands, assets: Res<UiAssets>) {
    commands.spawn(spawn_wave_ui(
        assets.spawnbtn.clone(),
        assets.spawnbtn_layout.clone(),
    ));
}

pub fn add_waves(mut wave_manager: ResMut<WaveManager>) {
    *wave_manager = WaveManager {
        upcoming_waves: test_waves(),
        ..Default::default()
    };
}

pub fn tick_wave_timer(mut wave_manager: ResMut<WaveManager>, time: Res<Time>) {
    wave_manager.wave_timer.tick(time.delta());
}

pub fn spawn_next_wave(
    mut wave_manager: ResMut<WaveManager>,
    mut commands: Commands,
    start_loc: Query<&Transform, With<StartNode>>,
    sfx: Res<SoundEffects>,
) {
    if wave_manager.wave_timer.finished() {
        if let (Some(wave), Ok(loc)) = (wave_manager.current_wave.as_mut(), start_loc.single()) {
            if let Some((group, duration)) = wave.0.pop_front() {
                for enemy in group.0.iter() {
                    commands.compose(enemy.clone() + loc.clone().store());
                }
                wave_manager.wave_timer.set_duration(duration);
                wave_manager.wave_timer.reset();
                commands.spawn(sound_effect(sfx.enemy_spawn_sfx.clone()));
            } else {
                wave_manager.current_wave = None;
            }
        }
    }
}

impl Default for WaveManager {
    fn default() -> Self {
        Self {
            current_wave: Default::default(),
            upcoming_waves: Default::default(),
            wave_timer: Timer::new(Duration::from_secs(1), bevy::time::TimerMode::Once),
        }
    }
}

impl WaveManager {
    pub fn remaining_waves(&self) -> usize {
        self.upcoming_waves.len()
    }
}

impl From<Vec<ComponentTree>> for Group {
    fn from(value: Vec<ComponentTree>) -> Self {
        Self(value)
    }
}

impl From<Vec<(Vec<ComponentTree>, f32)>> for Wave {
    fn from(value: Vec<(Vec<ComponentTree>, f32)>) -> Self {
        Wave(
            value
                .iter()
                .map(|(group, delay)| (group.clone().into(), Duration::from_secs_f32(*delay)))
                .collect(),
        )
    }
}

#[derive(Component)]
struct SpawnButtonMarker;

fn spawn_wave_ui(icon: Handle<Image>, layout: Handle<TextureAtlasLayout>) -> impl Bundle {
    (
        Name::new("Spawn"),
        StateScoped(Screen::Gameplay),
        // Button,
        Node {
            top: Val::Px(35.),
            left: Val::Px(60.),
            position_type: PositionType::Relative,
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        children![(
            Node {
                top: Val::Px(0.),
                left: Val::Px(0.),
                padding: UiRect::all(Val::Px(4.0)),
                width: Val::Px(128.0),
                height: Val::Px(128.0),
                ..default()
            },
            Pickable::default(),
            SpawnButtonMarker,
            ImageNode {
                image: icon,
                texture_atlas: Some(TextureAtlas {
                    layout: layout,
                    index: 0
                }),
                ..default()
            },
            children![widget::ui_font_with_node(
                "Next Wave",
                Node {
                    left: Val::Px(31.0),
                    top: Val::Px(48.0),
                    width: Val::Px(1.0),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
            ),]
        )],
    )
}

fn spawn_enter_observer(
    trigger: Trigger<Pointer<Over>>,
    mut spawn_button_marker: Query<&mut ImageNode, With<SpawnButtonMarker>>,
) {
    let Ok(mut image_node) = spawn_button_marker.get_mut(trigger.target) else {
        return;
    };
    if let Some(atlas) = &mut image_node.texture_atlas {
        atlas.index = 2;
    }
}
fn spawn_pressed_observer(
    trigger: Trigger<Pointer<Pressed>>,
    mut spawn_button_marker: Query<&mut ImageNode, With<SpawnButtonMarker>>,
) {
    let Ok(mut image_node) = spawn_button_marker.get_mut(trigger.target) else {
        return;
    };
    if let Some(atlas) = &mut image_node.texture_atlas {
        atlas.index = 1;
    }
}

fn spawn_released_observer(
    trigger: Trigger<Pointer<Released>>,
    mut spawn_button_marker: Query<&mut ImageNode, With<SpawnButtonMarker>>,
    mut wave_manager: ResMut<WaveManager>,
) {
    let Ok(mut image_node) = spawn_button_marker.get_mut(trigger.target) else {
        return;
    };
    if let Some(atlas) = &mut image_node.texture_atlas {
        atlas.index = 2;
    }
    if wave_manager.current_wave.is_none() {
        wave_manager.current_wave = wave_manager.upcoming_waves.pop_front();
    }
}

fn spawn_leave_observer(
    trigger: Trigger<Pointer<Out>>,
    mut spawn_button_marker: Query<&mut ImageNode, With<SpawnButtonMarker>>,
) {
    let Ok(mut image_node) = spawn_button_marker.get_mut(trigger.target) else {
        return;
    };
    if let Some(atlas) = &mut image_node.texture_atlas {
        atlas.index = 0;
    }
}

pub fn test_waves() -> VecDeque<Wave> {
    vec![
        //
        // Wave 1
        vec![
            (vec![basic_trooper()], 2.),
            (vec![basic_trooper(), turbo_trooper()], 0.),
        ]
        .into(),
        //
        // Wave 2
        vec![
            (vec![chonkus_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.),
        ]
        .into(),
        // Wave 3
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![basic_trooper(), turbo_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
        ]
        .into(),
        // Wave 4
        vec![
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
            (vec![chonkus_trooper(), basic_trooper()], 0.5),
        ].into()
    ]
    .into()
}
