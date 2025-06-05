use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        query::With,
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{common_conditions::input_just_pressed, keyboard::KeyCode},
    state::{condition::in_state, state::OnEnter},
    time::{Time, Timer},
    transform::components::Transform,
};
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    tree::ComponentTree,
};
use std::{collections::VecDeque, time::Duration};

use crate::{
    PausableSystems,
    level::components::StartNode,
    prefabs::enemies::{basic_trooper, chonkus_trooper, turbo_trooper},
    screens::Screen,
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

const WAVE_KEY: KeyCode = KeyCode::Space;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(WaveManager::default());
    app.add_systems(OnEnter(Screen::Gameplay), add_waves);
    app.add_systems(
        Update,
        (tick_wave_timer, spawn_next_wave)
            .in_set(PausableSystems)
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(Update, call_next_wave.run_if(input_just_pressed(WAVE_KEY)));
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
) {
    if wave_manager.wave_timer.finished() {
        if let (Some(wave), Ok(loc)) = (wave_manager.current_wave.as_mut(), start_loc.single()) {
            if let Some((group, duration)) = wave.0.pop_front() {
                for enemy in group.0.iter() {
                    commands.compose(enemy.clone() + loc.clone().store());
                }
                wave_manager.wave_timer.set_duration(duration);
                wave_manager.wave_timer.reset();
            } else {
                wave_manager.current_wave = None;
            }
        }
    }
}

pub fn call_next_wave(mut wave_manager: ResMut<WaveManager>) {
    if wave_manager.current_wave.is_none() {
        wave_manager.current_wave = wave_manager.upcoming_waves.pop_front();
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
    ]
    .into()
}
